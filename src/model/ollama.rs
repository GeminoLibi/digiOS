use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, error, warn};

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    model: String,
    #[serde(default)]
    response: String,
    done: bool,
    #[serde(default)]
    context: Vec<u64>,
}

/// Ollama API client for model inference
pub struct OllamaClient {
    base_url: String,
    model_name: String,
}

impl OllamaClient {
    pub fn new(model_name: String) -> Self {
        // Extract model name from "ollama:modelname" format
        let name = if model_name.starts_with("ollama:") {
            model_name.strip_prefix("ollama:").unwrap().to_string()
        } else {
            model_name
        };
        
        Self {
            base_url: "http://localhost:11434".to_string(),
            model_name: name,
        }
    }

    /// Check if Ollama is running
    pub async fn check_available(&self) -> Result<bool> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/tags", self.base_url);
        
        match client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => {
                warn!("Ollama not available at {}", self.base_url);
                Ok(false)
            }
        }
    }

    /// Generate text using Ollama
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        info!("Calling Ollama model '{}' with prompt ({} chars)", self.model_name, prompt.len());
        
        let client = reqwest::Client::new();
        let url = format!("{}/api/generate", self.base_url);
        
        let request = OllamaRequest {
            model: self.model_name.clone(),
            prompt: prompt.to_string(),
            stream: false,
            options: Some(OllamaOptions {
                temperature: Some(0.7),
                top_p: Some(0.9),
                top_k: Some(40),
            }),
        };
        
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("Ollama API error ({}): {}", status, text));
        }
        
        let ollama_response: OllamaResponse = response.json().await?;
        
        info!("Ollama response received ({} chars)", ollama_response.response.len());
        Ok(ollama_response.response)
    }

    /// List available models
    pub async fn list_models() -> Result<Vec<String>> {
        let client = reqwest::Client::new();
        let url = "http://localhost:11434/api/tags";
        
        #[derive(Deserialize)]
        struct ModelsResponse {
            models: Vec<ModelInfo>,
        }
        
        #[derive(Deserialize)]
        struct ModelInfo {
            name: String,
        }
        
        let response = client.get(url).send().await?;
        let models_response: ModelsResponse = response.json().await?;
        
        Ok(models_response.models.into_iter().map(|m| m.name).collect())
    }
}

