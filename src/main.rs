mod ai;

use ai::{query_ai_hint, select_ai_source};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use which::which;

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

        println!("{}", "💭 Validating with AI...".dimmed());

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
}