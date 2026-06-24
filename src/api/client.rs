use crate::error::{Error, Result};
use reqwest::Client;
use log::{debug, warn};

use super::models::{ChatMessage, ChatRequest, ChatResponse, ModelsResponse};

pub struct OllamaClient {
    http_client: Client,
    endpoint: String,
    api_key: String,
}

impl OllamaClient {
    pub fn new(endpoint: &str, api_key: &str) -> Result<Self> {
        if endpoint.is_empty() {
            return Err(Error::ConfigError("Endpoint cannot be empty".to_string()));
        }
        if api_key.is_empty() {
            return Err(Error::ConfigError("API key cannot be empty".to_string()));
        }

        Ok(OllamaClient {
            http_client: Client::new(),
            endpoint: endpoint.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        })
    }

    /// Fetch available models from the Ollama Cloud API
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.endpoint);
        debug!("Fetching models from: {}", url);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| Error::ApiError(format!("Failed to fetch models: {}", e)))?;

        let models_resp: ModelsResponse = response
            .json()
            .await
            .map_err(|e| Error::ApiError(format!("Failed to parse models: {}", e)))?;

        Ok(models_resp
            .models
            .into_iter()
            .map(|m| m.name)
            .collect())
    }

    /// Send a chat message to a model
    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<ChatResponse> {
        let url = format!("{}/api/chat", self.endpoint);
        debug!("Sending chat request to: {}", url);

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: false,
        };

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::ApiError(format!("Failed to send message: {}", e)))?;

        let chat_resp: ChatResponse = response
            .json()
            .await
            .map_err(|e| Error::ApiError(format!("Failed to parse response: {}", e)))?;

        Ok(chat_resp)
    }

    /// Get a streaming response iterator (collects chunks)
    /// Note: For MVP, we'll collect the full response. Streaming can be optimized later.
    #[allow(dead_code)]
    pub async fn get_streaming_response(
        &self,
        model: &str,
        messages: Vec<ChatMessage>,
    ) -> Result<String> {
        let url = format!("{}/api/chat", self.endpoint);
        debug!("Getting streaming response from: {}", url);

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: true,
        };

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::ApiError(format!("Failed to start stream: {}", e)))?;

        if !response.status().is_success() {
            warn!("API returned status: {}", response.status());
            return Err(Error::ApiError(format!(
                "API error: {}",
                response.status()
            )));
        }

        let text = response.text().await?;
        Ok(text)
    }

    /// Test API connectivity
    #[allow(dead_code)]
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.endpoint);
        match self
            .http_client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                warn!("Health check failed: {}", e);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = OllamaClient::new("http://localhost:11434", "test-key");
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_empty_endpoint() {
        let client = OllamaClient::new("", "test-key");
        assert!(client.is_err());
    }

    #[test]
    fn test_client_empty_api_key() {
        let client = OllamaClient::new("http://localhost:11434", "");
        assert!(client.is_err());
    }
}
