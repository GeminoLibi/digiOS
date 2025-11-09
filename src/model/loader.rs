use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, warn};
use crate::model::ollama::OllamaClient;

/// Model Loader - Handles loading and running AI models
pub struct ModelLoader {
    path: PathBuf,
    loaded: bool,
    ollama_client: Option<OllamaClient>,
    model_name: String,
}

impl ModelLoader {
    pub async fn new(path: &PathBuf) -> Result<Self> {
        info!("Initializing model loader for: {:?}", path);
        
        // Check if this is an Ollama model
        let path_str = path.to_string_lossy();
        let (ollama_client, model_name) = if path_str.starts_with("ollama://") {
            let model_name = path_str.strip_prefix("ollama://").unwrap().to_string();
            let client = OllamaClient::new(model_name.clone());
            
            // Check if Ollama is available
            if client.check_available().await.unwrap_or(false) {
                info!("Ollama detected - using API for model: {}", model_name);
                (Some(client), format!("ollama:{}", model_name))
            } else {
                warn!("Ollama path detected but Ollama not available");
                (None, model_name)
            }
        } else {
            (None, path_str.to_string())
        };
        
        Ok(Self {
            path: path.clone(),
            loaded: false,
            ollama_client,
            model_name,
        })
    }

    pub async fn load(&mut self) -> Result<()> {
        info!("Loading model from: {:?}", self.path);
        
        // For Ollama models, just verify availability
        if let Some(ref client) = self.ollama_client {
            if !client.check_available().await? {
                return Err(anyhow::anyhow!("Ollama is not running. Please start Ollama first."));
            }
            info!("Ollama model ready: {}", self.model_name);
        } else {
            // For local file models, would load here
            info!("Local model file: {:?}", self.path);
        }
        
        self.loaded = true;
        info!("Model loaded");
        Ok(())
    }

    pub async fn infer(&self, prompt: &str) -> Result<String> {
        if !self.loaded {
            return Err(anyhow::anyhow!("Model not loaded"));
        }

        // Use Ollama if available
        if let Some(ref client) = self.ollama_client {
            info!("Calling Ollama for inference");
            return client.generate(prompt).await;
        }
        
        // Fallback for other model types
        warn!("Model inference not fully implemented for: {:?}", self.path);
        Ok(format!("[Model response to: {}]", prompt))
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
}

impl Clone for ModelLoader {
    fn clone(&self) -> Self {
        // Note: OllamaClient doesn't need to be cloned as it's just a config
        // We'll recreate it if needed
        let path_str = self.path.to_string_lossy();
        let (ollama_client, model_name) = if path_str.starts_with("ollama://") {
            let model_name = path_str.strip_prefix("ollama://").unwrap().to_string();
            let client = OllamaClient::new(model_name.clone());
            (Some(client), format!("ollama:{}", model_name))
        } else {
            (None, path_str.to_string())
        };
        
        Self {
            path: self.path.clone(),
            loaded: self.loaded,
            ollama_client,
            model_name,
        }
    }
}

