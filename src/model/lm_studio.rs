use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, warn, error};

/// LM Studio model client - Uses LM Studio's local server API
pub struct LMStudioClient {
    model_path: PathBuf,
    base_url: String,
}

impl LMStudioClient {
    pub fn new(model_path: PathBuf) -> Self {
        Self {
            model_path,
            base_url: "http://localhost:1234".to_string(), // LM Studio default port
        }
    }

    /// Check if LM Studio server is running
    pub async fn check_available(&self) -> Result<bool> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/models", self.base_url);
        
        match client.get(&url).timeout(std::time::Duration::from_secs(2)).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => {
                warn!("LM Studio server not available at {}", self.base_url);
                Ok(false)
            }
        }
    }

    /// Generate text using LM Studio API (OpenAI-compatible)
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        info!("Calling LM Studio for model at: {:?}", self.model_path);
        
        let client = reqwest::Client::new();
        let url = format!("{}/v1/chat/completions", self.base_url);
        
        // LM Studio uses OpenAI-compatible API
        #[derive(serde::Serialize)]
        struct ChatRequest {
            model: String,
            messages: Vec<Message>,
            temperature: f32,
            stream: bool,
        }
        
        #[derive(serde::Serialize)]
        struct Message {
            role: String,
            content: String,
        }
        
        // Get model name from path
        let model_name = self.model_path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("model")
            .to_string();
        
        let request = ChatRequest {
            model: model_name.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: 0.7,
            stream: false,
        };
        
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("LM Studio API error ({}): {}", status, text));
        }
        
        #[derive(serde::Deserialize)]
        struct ChatResponse {
            choices: Vec<Choice>,
        }
        
        #[derive(serde::Deserialize)]
        struct Choice {
            message: ResponseMessage,
        }
        
        #[derive(serde::Deserialize)]
        struct ResponseMessage {
            content: String,
        }
        
        let chat_response: ChatResponse = response.json().await?;
        
        if let Some(choice) = chat_response.choices.first() {
            let content = choice.message.content.clone();
            info!("LM Studio response received ({} chars)", content.len());
            Ok(content)
        } else {
            Err(anyhow::anyhow!("No response from LM Studio"))
        }
    }
}

