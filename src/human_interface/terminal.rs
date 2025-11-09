use anyhow::Result;
use tracing::info;
use std::io::{self, Write};

/// Terminal Interface - Allows humans to interact with digiOS
pub struct TerminalInterface;

impl TerminalInterface {
    pub async fn start() -> Result<()> {
        info!("Starting terminal interface");
        
        println!("\n=== digiOS Terminal Interface ===");
        println!("Type 'help' for commands, 'exit' to quit\n");
        
        loop {
            print!("digiOS> ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let command = input.trim();
            
            if command.is_empty() {
                continue;
            }
            
            match command {
                "exit" | "quit" => {
                    println!("Goodbye!");
                    break;
                }
                "help" => {
                    Self::show_help();
                }
                "status" => {
                    Self::show_status().await?;
                }
                "capabilities" => {
                    Self::show_capabilities().await?;
                }
                _ => {
                    println!("Unknown command: {}. Type 'help' for available commands.", command);
                }
            }
        }
        
        Ok(())
    }

    fn show_help() {
        println!("\nAvailable commands:");
        println!("  help         - Show this help message");
        println!("  status       - Show system status");
        println!("  capabilities - Show system capabilities");
        println!("  exit/quit    - Exit terminal interface");
        println!();
    }

    async fn show_status() -> Result<()> {
        println!("\nSystem Status:");
        println!("  Status: Running");
        println!("  Model: Loaded");
        println!("  Self-Improvement: Active");
        println!();
        Ok(())
    }

    async fn show_capabilities() -> Result<()> {
        println!("\nSystem Capabilities:");
        println!("  - Action execution");
        println!("  - Vision processing");
        println!("  - State management");
        println!("  - Task planning");
        println!("  - Self-improvement");
        println!();
        Ok(())
    }
}

