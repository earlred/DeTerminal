// src/ai.rs

use dialoguer::{Select, theme::ColorfulTheme};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::sync::Mutex;
use std::time::Duration;

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub enum AISource {
    OpenAI,
    Ollama,
    None,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug)]
pub struct AIHint {
    pub text: String,
    pub command: Option<String>,
}

lazy_static! {
    pub static ref AI_BACKEND: Mutex<AISource> = Mutex::new(AISource::None);
    pub static ref OLLAMA_MODEL: Mutex<String> = Mutex::new("llama3".to_string());
}

pub fn select_ai_source() {
    let has_openai = env::var("OPENAI_API_KEY").is_ok();
    let has_ollama = check_ollama();

    let choices = [
        (has_openai, "OpenAI (GPT-4)", AISource::OpenAI),
        (has_ollama, "Ollama (local)", AISource::Ollama),
    ]
    .into_iter()
    .filter(|(available, _, _)| *available)
    .collect::<Vec<_>>();

    if choices.is_empty() {
        eprintln!("❌ No AI sources available. Set OPENAI_API_KEY or start Ollama.");
        std::process::exit(1);
    }

    let items: Vec<&str> = choices.iter().map(|(_, label, _)| *label).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your AI provider:")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    let selected = &choices[selection].2;
    *AI_BACKEND.lock().unwrap() = selected.clone();

    // Prompt for Ollama model
    if let AISource::Ollama = selected {
        let client = Client::new();
        let models_resp =
            client.get("http://localhost:11434/api/tags").timeout(Duration::from_secs(2)).send();

        if let Ok(resp) = models_resp {
            if let Ok(json) = resp.json::<serde_json::Value>() {
                if let Some(models) = json["models"].as_array() {
                    let names: Vec<&str> =
                        models.iter().filter_map(|m| m["name"].as_str()).collect();

                    if !names.is_empty() {
                        let model_index = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Choose a local Ollama model:")
                            .items(&names)
                            .default(0)
                            .interact()
                            .unwrap();

                        *OLLAMA_MODEL.lock().unwrap() = names[model_index].to_string();
                    }
                }
            }
        }
    }
}

fn check_ollama() -> bool {
    let client = Client::new();
    client
        .get("http://localhost:11434/api/tags")
        .timeout(Duration::from_secs(1))
        .send()
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

pub fn query_ai_hint(user_input: &str) -> Result<AIHint, Box<dyn std::error::Error>> {
    let platform = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let prompt = format!(
        "The user is running {} ({}). They typed this in their terminal:\n\n\"{}\"\n\n\
        If the command is a request for help or information, suggest specific, actionable commands.\n\
        For example, if they type 'help', suggest 'man bash' or 'help' (for bash) or 'compgen -c' to list commands.\n\
        Do not use placeholders like [command] - suggest actual commands.\n\
        If the command is valid, explain it and repeat it exactly.\n\
        If it's invalid, infer the correct command and output the result in this format:\n\n\
        Explanation: ...\n\
        Command: <shell_command>",
        platform, arch, user_input
    );

    let backend = AI_BACKEND.lock().unwrap().clone();
    let full_text = match backend {
        AISource::OpenAI => query_openai(&prompt)?,
        AISource::Ollama => query_ollama(&prompt)?,
        AISource::None => return Err("No AI backend selected".into()),
    };

    let mut explanation = String::new();
    let mut command = None;

    for line in full_text.lines() {
        let line = line.trim();
        if line.to_lowercase().starts_with("command:") {
            let cmd = line[8..].trim();

            let cleaned = cmd
                .trim_start_matches("```bash")
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim_matches(&['`', '"', '\''][..])
                .trim();

            if !cleaned.is_empty() {
                command = Some(cleaned.to_string());
            }
        } else if line.to_lowercase().starts_with("explanation:") {
            explanation = line[12..].trim().to_string();
        } else {
            explanation.push_str(" ");
            explanation.push_str(line);
        }
    }

    Ok(AIHint { text: explanation.trim().to_string(), command })
}

fn query_openai(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "model": "gpt-4",
            "messages": [
                {"role": "system", "content": "You are a shell assistant. Only suggest shell commands."},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.0
        }))
        .send()?;

    let parsed: OpenAIResponse = response.json()?;
    Ok(parsed.choices.get(0).map(|c| c.message.content.trim().to_string()).unwrap_or_default())
}

fn query_ollama(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let model = OLLAMA_MODEL.lock().unwrap().clone();
    let client = Client::new();

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "unknown".into());

    let full_prompt = format!(
        "You are a helpful shell assistant.\n\
The user is running {os} ({arch}) using {shell}.\n\
If the user input is valid, explain it and repeat it exactly.\n\
If it's invalid, infer the correct command and output the result in the following format:\n\
\nExplanation: ...\nCommand: <shell_command>\n\nUser input:\n{prompt}\n"
    );

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": model,
            "prompt": full_prompt,
            "stream": false
        }))
        .send()?;

    let result: serde_json::Value = response.json()?;
    Ok(result["response"].as_str().unwrap_or("").trim().to_string())
}
