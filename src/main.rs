use log::info;

mod api;
mod config;
mod error;
mod storage;
mod conversation;
mod cli;

use cli::CliHandler;
use conversation::ConversationManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting Ollama Cloud Client...");

    // Load configuration from environment
    let config = config::Config::from_env()?;
    info!("Configuration loaded: {:?}", config);

    // Initialize storage
    let db = storage::Database::new(&config.db_path).await?;
    info!("Database initialized at: {}", config.db_path);

    // Initialize API client
    let client = api::OllamaClient::new(&config.api_endpoint, &config.api_key)?;
    info!("Ollama API client initialized");

    // Initialize conversation manager
    let manager = ConversationManager::new(client, db).await?;
    info!("Conversation manager initialized");

    // Start CLI interface
    let mut cli = CliHandler::new(manager);
    cli.run().await?;

    Ok(())
}
