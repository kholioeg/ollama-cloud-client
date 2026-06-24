use crate::conversation::ConversationManager;
use crate::error::Result;
use log::error;
use std::io::{self, Write};

pub struct CliHandler {
    manager: ConversationManager,
}

impl CliHandler {
    pub fn new(manager: ConversationManager) -> Self {
        CliHandler { manager }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("\n╔══════════════════════════════════════════════════════╗");
        println!("║     Ollama Cloud Client - CLI Interface (v0.1)      ║");
        println!("╚══════════════════════════════════════════════════════╝\n");

        // Test API connection and fetch models
        match self.manager.list_models().await {
            Ok(models) => {
                println!("✓ Connected to Ollama Cloud!");
                println!("Available models:");
                for (i, model) in models.iter().enumerate() {
                    println!("  {}. {}", i + 1, model);
                }
            }
            Err(e) => {
                error!("Failed to connect to Ollama Cloud: {}", e);
                println!("✗ Failed to connect to Ollama Cloud");
                println!("  Error: {}", e);
                println!("\n  Ensure your .env file is configured with:");
                println!("  - OLLAMA_API_ENDPOINT");
                println!("  - OLLAMA_API_KEY");
                return Ok(());
            }
        }

        // Get model selection
        let models = self.manager.list_models().await?;
        if models.is_empty() {
            println!("No models available.");
            return Ok(());
        }

        let selected_model = self.select_model(&models)?;
        
        // Start conversation
        let conv_id = self.manager.start_conversation("CLI Chat", &selected_model).await?;
        println!("\n✓ Started conversation (ID: {})\n", conv_id);

        // Chat loop
        self.chat_loop().await?;

        Ok(())
    }

    fn select_model(&self, models: &[String]) -> Result<String> {
        loop {
            println!("\nSelect a model by number: ");
            io::stdout().flush().ok();

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim().parse::<usize>() {
                Ok(idx) if idx > 0 && idx <= models.len() => {
                    let selected = models[idx - 1].clone();
                    println!("Selected model: {}", selected);
                    return Ok(selected);
                }
                _ => {
                    println!("Invalid selection. Please enter a number between 1 and {}.", models.len());
                }
            }
        }
    }

    async fn chat_loop(&mut self) -> Result<()> {
        println!("Type 'exit' to quit, 'history' to see chat history\n");

        loop {
            print!("You: ");
            io::stdout().flush().ok();

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            if input.eq_ignore_ascii_case("exit") {
                println!("\n✓ Goodbye!");
                break;
            }

            if input.eq_ignore_ascii_case("history") {
                self.show_history().await?;
                continue;
            }

            if input.eq_ignore_ascii_case("models") {
                match self.manager.list_models().await {
                    Ok(models) => {
                        println!("\nAvailable models:");
                        for model in models {
                            println!("  - {}", model);
                        }
                    }
                    Err(e) => {
                        println!("Error fetching models: {}", e);
                    }
                }
                continue;
            }

            print!("Assistant: ");
            io::stdout().flush().ok();

            match self.manager.send_message(input).await {
                Ok(response) => {
                    println!("{}\n", response);
                }
                Err(e) => {
                    error!("Error sending message: {}", e);
                    println!("Error: {}\n", e);
                }
            }
        }

        Ok(())
    }

    async fn show_history(&self) -> Result<()> {
        let conversations = self.manager.get_history().await?;
        
        if conversations.is_empty() {
            println!("\nNo conversation history found.\n");
            return Ok(());
        }

        println!("\n╔════════════════════════════════╗");
        println!("║      Conversation History      ║");
        println!("╚════════════════════════════════╝");

        for (id, title) in conversations {
            println!("ID: {} | Title: {}", id, title);
        }
        println!();

        Ok(())
    }
}
