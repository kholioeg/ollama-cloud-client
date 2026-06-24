use crate::api::{OllamaClient, Message, MessageRole, ChatMessage};
use crate::storage::Database;
use crate::error::Result;
use uuid::Uuid;
use chrono::Utc;
use log::{info, debug};

pub struct ConversationManager {
    client: OllamaClient,
    db: Database,
    current_conversation_id: Option<String>,
    current_model: Option<String>,
}

impl ConversationManager {
    pub async fn new(client: OllamaClient, db: Database) -> Result<Self> {
        Ok(ConversationManager {
            client,
            db,
            current_conversation_id: None,
            current_model: None,
        })
    }

    /// Get available models from Ollama Cloud
    pub async fn list_models(&self) -> Result<Vec<String>> {
        info!("Fetching available models...");
        self.client.list_models().await
    }

    /// Start a new conversation
    pub async fn start_conversation(&mut self, title: &str, model: &str) -> Result<String> {
        info!("Starting new conversation: {} with model: {}", title, model);
        
        let conv_id = self.db.create_conversation(title, model).await?;
        self.current_conversation_id = Some(conv_id.clone());
        self.current_model = Some(model.to_string());

        Ok(conv_id)
    }

    /// Send a message in the current conversation
    pub async fn send_message(&self, content: &str) -> Result<String> {
        let conv_id = self
            .current_conversation_id
            .as_ref()
            .ok_or_else(|| crate::error::Error::ConfigError("No active conversation".to_string()))?;

        let model = self
            .current_model
            .as_ref()
            .ok_or_else(|| crate::error::Error::ConfigError("No model selected".to_string()))?;

        info!("Sending message in conversation: {}", conv_id);
        debug!("Message content: {}", content);

        // Save user message
        let user_msg = Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conv_id.clone(),
            role: MessageRole::User,
            content: content.to_string(),
            created_at: Utc::now(),
        };

        self.db.save_message(conv_id, &user_msg).await?;

        // Send to API
        let chat_messages = vec![ChatMessage {
            role: "user".to_string(),
            content: content.to_string(),
        }];

        let response = self.client.chat(model, chat_messages).await?;

        // Save assistant response
        let assistant_msg = Message {
            id: Uuid::new_v4().to_string(),
            conversation_id: conv_id.clone(),
            role: MessageRole::Assistant,
            content: response.message.content.clone(),
            created_at: Utc::now(),
        };

        self.db.save_message(conv_id, &assistant_msg).await?;

        Ok(response.message.content)
    }

    /// Get conversation history
    pub async fn get_history(&self) -> Result<Vec<(String, String)>> {
        self.db.list_conversations().await
    }

    /// Set the active conversation
    #[allow(dead_code)]
    pub fn set_conversation(&mut self, conv_id: String, model: String) {
        self.current_conversation_id = Some(conv_id);
        self.current_model = Some(model);
    }

    /// Get current conversation info
    #[allow(dead_code)]
    pub fn current_info(&self) -> Option<(String, String)> {
        match (&self.current_conversation_id, &self.current_model) {
            (Some(id), Some(model)) => Some((id.clone(), model.clone())),
            _ => None,
        }
    }
}
