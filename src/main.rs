mod ai;

use ai::{query_ai_hint, select_ai_source};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use which::which;

lazy_static::lazy_static! {
    static ref VALID_COMMANDS: HashSet<String> = {
        let mut commands = HashSet::new();
        if let Ok(contents) = fs::read_to_string("src/commands.json") {
            if let Ok(json) = serde_json::from_str::<Value>(&contents) {
                // Add common commands
                if let Some(common) = json.get("common") {
                    if let Some(obj) = common.as_object() {
                        for (_, value) in obj {
                            if let Some(arr) = value.as_array() {
                                for cmd in arr {
                                    if let Some(cmd_str) = cmd.as_str() {
                                        commands.insert(cmd_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                }

                // Add shell-specific commands
                let shell = std::env::var("SHELL").unwrap_or_else(|_| {
                    if cfg!(windows) {
                        if std::env::var("PSModulePath").is_ok() {
                            "powershell".to_string()
                        } else {
                            "cmd".to_string()
                        }
                    } else {
                        "bash".to_string()
                    }
                });
                
                let shell_type = if shell.contains("zsh") {
                    "zsh"
                } else if shell.contains("powershell") || shell.contains("pwsh") {
                    "powershell"
                } else if shell.contains("cmd") {
                    "cmd"
                } else {
                    "bash"
                };
                
                if let Some(shell_commands) = json.get(shell_type) {
                    if let Some(obj) = shell_commands.as_object() {
                        for (_, value) in obj {
                            if let Some(arr) = value.as_array() {
                                for cmd in arr {
                                    if let Some(cmd_str) = cmd.as_str() {
                                        commands.insert(cmd_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        commands
    };
}

fn is_valid_command(input: &str) -> bool {
    // Split the input into command and arguments
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return false;
    }

    // Check if the base command is in our list
    let base_cmd = parts[0];
    VALID_COMMANDS.contains(base_cmd)
}

fn print_banner() {
    println!(
        "{}\n{}\n{}",
        "=".repeat(40),
        "DeTerminal: Semantic Terminal AI Assistant".bold().green(),
        "=".repeat(40),
    );
}

fn run_command(command: &str) {
    // First, try splitting normally
    match shell_words::split(command) {
        Ok(parts) if !parts.is_empty() => {
            let args: Vec<&str> = parts.iter().map(AsRef::as_ref).collect();
            if let Some((program, args)) = args.split_first() {
                // If no pipe or redirect, run normally
                if !command.contains('|') && !command.contains('>') && which(program).is_ok() {
                    let status = Command::new(program)
                        .args(args)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .status();

                    if let Err(e) = status {
                        eprintln!("{}: {}", "❌ Execution failed".red(), e);
                    }
                    return;
                }
            }
        }
        _ => {}
    }

    // Fallback: shell execution for complex commands
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    if let Err(e) = status {
        eprintln!("{}: {}", "❌ Execution failed".red(), e);
    }
}

fn main() {
    print_banner();
    select_ai_source();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("{} ", "DeTerminal >".blue());
        stdout.flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("{}", "❌ Failed to read input".red());
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }

        // First try to execute the command directly if it's in our valid commands list
        if is_valid_command(input) {
            let status = Command::new("sh")
                .arg("-c")
                .arg(input)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            match status {
                Ok(exit_status) => {
                    if !exit_status.success() {
                        eprintln!("{}: Command exited with status {}", "❌ Command failed".red(), exit_status);
                        println!("{}", "💭 Validating with AI...".dimmed());
                        handle_ai_validation(input);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "❌ Execution failed".red(), e);
                    println!("{}", "💭 Validating with AI...".dimmed());
                    handle_ai_validation(input);
                }
            }
        } else {
            println!("{}", "💭 Validating with AI...".dimmed());
            handle_ai_validation(input);
        }
    }
}

fn handle_ai_validation(input: &str) {
    match query_ai_hint(input) {
        Ok(hint) => {
            let explanation_lower = hint.text.to_lowercase();
            let is_valid = explanation_lower.contains("✅")
                || explanation_lower.contains("is correct")
                || explanation_lower.contains("already correct")
                || explanation_lower.contains("no need for correction")
                || explanation_lower.contains("is valid");

            if is_valid && (hint.command.is_none() || hint.command.as_deref() == Some(input)) {
                run_command(input);
            } else if let Some(command) = hint.command {
                // Split multiple commands if they exist
                let commands: Vec<&str> = command.split(" or ").collect();
                
                if commands.len() > 1 {
                    println!("{}", "🤖 I found multiple possible commands:".cyan());
                    for (i, cmd) in commands.iter().enumerate() {
                        println!("{}. {}", i + 1, cmd.trim());
                    }
                    println!("{} {}", "ℹ️ ".dimmed(), hint.text.dimmed());
                    
                    let selection = dialoguer::Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select a command to run")
                        .items(&commands)
                        .default(0)
                        .interact()
                        .unwrap_or(0);
                        
                    run_command(commands[selection].trim());
                } else {
                    println!(
                        "{} {}",
                        "🤖 I think you meant to run:".cyan(),
                        format!("`{}`", command)
                    );
                    println!("{} {}", "ℹ️ ".dimmed(), hint.text.dimmed());

                    let confirm = Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("❓ Do you want to run it?")
                        .interact()
                        .unwrap_or(false);

                    if confirm {
                        run_command(&command);
                    }
                }
            } else {
                println!("{} {}", "🧠 AI Explanation:".cyan(), hint.text);
            }
        }
        Err(err) => {
            eprintln!("{} {}", "❌ Failed to reach AI:".red(), err);
        }
    }
}