use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, warn};
use crate::model::ollama::OllamaClient;
use crate::model::python_ollama::PythonOllamaClient;

/// Model Loader - Handles loading and running AI models
pub struct ModelLoader {
    path: PathBuf,
    loaded: bool,
    ollama_client: Option<OllamaClient>,
    python_ollama_client: Option<PythonOllamaClient>,
    model_name: String,
}

impl ModelLoader {
    pub async fn new(path: &PathBuf) -> Result<Self> {
        info!("Initializing model loader for: {:?}", path);
        
        // Check if this is an Ollama model
        let path_str = path.to_string_lossy();
        let (ollama_client, python_ollama_client, model_name) = if path_str.starts_with("ollama://") {
            let model_name = path_str.strip_prefix("ollama://").unwrap().to_string();
            let full_name = format!("ollama:{}", model_name);
            
            // Try Ollama server first
            let ollama_client = OllamaClient::new(model_name.clone());
            let ollama_available = ollama_client.check_available().await.unwrap_or(false);
            
            if ollama_available {
                info!("Ollama server detected - using API for model: {}", model_name);
                (Some(ollama_client), None, full_name)
            } else {
                // Fallback to Python ollama client
                warn!("Ollama server not available, trying Python ollama client");
                let python_client = PythonOllamaClient::new(model_name.clone());
                let python_available = python_client.check_available().await.unwrap_or(false);
                
                if python_available {
                    info!("Python ollama client detected - using for model: {}", model_name);
                    (None, Some(python_client), full_name)
                } else {
                    warn!("Neither Ollama server nor Python client available");
                    (None, None, model_name)
                }
            }
        } else {
            (None, None, path_str.to_string())
        };
        
        Ok(Self {
            path: path.clone(),
            loaded: false,
            ollama_client,
            python_ollama_client,
            model_name,
        })
    }

    pub async fn load(&mut self) -> Result<()> {
        info!("Loading model from: {:?}", self.path);
        
        // For Ollama models, verify availability
        if let Some(ref client) = self.ollama_client {
            if !client.check_available().await? {
                return Err(anyhow::anyhow!("Ollama server is not running. Please start Ollama first."));
            }
            info!("Ollama server model ready: {}", self.model_name);
        } else if let Some(ref client) = self.python_ollama_client {
            if !client.check_available().await? {
                return Err(anyhow::anyhow!("Python ollama client is not available."));
            }
            info!("Python ollama client model ready: {}", self.model_name);
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

        // Use Ollama server if available
        if let Some(ref client) = self.ollama_client {
            info!("Calling Ollama server for inference");
            return client.generate(prompt).await;
        }
        
        // Use Python ollama client as fallback
        if let Some(ref client) = self.python_ollama_client {
            info!("Calling Python ollama client for inference");
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
        let (ollama_client, python_ollama_client, model_name) = if path_str.starts_with("ollama://") {
            let model_name = path_str.strip_prefix("ollama://").unwrap().to_string();
            let full_name = format!("ollama:{}", model_name);
            let ollama_client = OllamaClient::new(model_name.clone());
            let python_client = PythonOllamaClient::new(model_name.clone());
            (Some(ollama_client), Some(python_client), full_name)
        } else {
            (None, None, path_str.to_string())
        };
        
        Self {
            path: self.path.clone(),
            loaded: self.loaded,
            ollama_client,
            python_ollama_client,
            model_name,
        }
    }
}

