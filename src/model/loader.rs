use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, warn};
use crate::model::ollama::OllamaClient;
use crate::model::python_ollama::PythonOllamaClient;
use crate::model::lm_studio::LMStudioClient;
use crate::model::huggingface::HuggingFaceClient;

/// Model Loader - Handles loading and running AI models
pub struct ModelLoader {
    path: PathBuf,
    loaded: bool,
    ollama_client: Option<OllamaClient>,
    python_ollama_client: Option<PythonOllamaClient>,
    lm_studio_client: Option<LMStudioClient>,
    huggingface_client: Option<HuggingFaceClient>,
    model_name: String,
}

impl ModelLoader {
    pub async fn new(path: &PathBuf) -> Result<Self> {
        info!("Initializing model loader for: {:?}", path);
        
        let path_str = path.to_string_lossy();
        let mut ollama_client = None;
        let mut python_ollama_client = None;
        let mut lm_studio_client = None;
        let mut huggingface_client = None;
        let model_name;
        
        // Check for Ollama
        if path_str.starts_with("ollama://") {
            let model_name_raw = path_str.strip_prefix("ollama://").unwrap().to_string();
            model_name = format!("ollama:{}", model_name_raw);
            
            // Try Ollama server first
            let client = OllamaClient::new(model_name_raw.clone());
            let ollama_available = client.check_available().await.unwrap_or(false);
            
            if ollama_available {
                info!("Ollama server detected");
                ollama_client = Some(client);
            } else {
                // Fallback to Python ollama client
                warn!("Ollama server not available, trying Python ollama client");
                let python_client = PythonOllamaClient::new(model_name_raw.clone());
                if python_client.check_available().await.unwrap_or(false) {
                    info!("Python ollama client detected");
                    python_ollama_client = Some(python_client);
                }
            }
        }
        // Check for LM Studio model (local file path)
        else if path.exists() && path.is_file() {
            // Check if this is a placeholder file (skip it)
            let is_placeholder = if let Ok(contents) = std::fs::read_to_string(path) {
                contents.contains("digiOS Model Placeholder") || 
                contents.contains("TODO: Implement actual code generation") ||
                contents.contains("Status: Placeholder")
            } else {
                false
            };
            
            if is_placeholder {
                warn!("Skipping placeholder model file: {:?}", path);
                model_name = path_str.to_string();
            } else {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if matches!(ext, "gguf" | "bin" | "safetensors" | "pt" | "pth") {
                    model_name = path.file_stem()
                        .and_then(|n| n.to_str())
                        .unwrap_or("model")
                        .to_string();
                    
                    // Try LM Studio first (if server is running)
                    let lm_client = LMStudioClient::new(path.clone());
                    if lm_client.check_available().await.unwrap_or(false) {
                        info!("LM Studio server detected for model: {:?}", path);
                        lm_studio_client = Some(lm_client);
                    } else {
                        // Check if it's in Hugging Face cache
                        let path_str_lower = path_str.to_lowercase();
                        if path_str_lower.contains("huggingface") || path_str_lower.contains(".cache") {
                            let hf_client = HuggingFaceClient::new(path.clone());
                            if hf_client.check_available().await.unwrap_or(false) {
                                info!("Hugging Face model detected: {:?}", path);
                                huggingface_client = Some(hf_client);
                            }
                        }
                    }
                } else {
                    model_name = path_str.to_string();
                }
            }
        } else {
            model_name = path_str.to_string();
        }
        
        Ok(Self {
            path: path.clone(),
            loaded: false,
            ollama_client,
            python_ollama_client,
            lm_studio_client,
            huggingface_client,
            model_name,
        })
    }

    pub async fn load(&mut self) -> Result<()> {
        info!("Loading model from: {:?}", self.path);
        
        // Verify model availability based on type
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
        } else if let Some(ref client) = self.lm_studio_client {
            // Check if LM Studio is actually running before trying to use it
            if !client.check_available().await? {
                warn!("LM Studio server is not running. Model file exists but server unavailable.");
                warn!("Please start LM Studio and load a model, or use a different model provider.");
                return Err(anyhow::anyhow!(
                    "LM Studio server is not running.\n\
                    To use LM Studio models:\n\
                    1. Start LM Studio application\n\
                    2. Load a model in LM Studio\n\
                    3. Make sure the local server is enabled (port 1234)\n\
                    \n\
                    Or select a different model during setup."
                ));
            }
            info!("LM Studio model ready: {}", self.model_name);
        } else if let Some(ref client) = self.huggingface_client {
            if !client.check_available().await? {
                return Err(anyhow::anyhow!("Hugging Face transformers library is not available. Install with: pip install transformers"));
            }
            info!("Hugging Face model ready: {}", self.model_name);
        } else {
            // Check if this is a placeholder file
            if self.path.exists() && self.path.is_file() {
                if let Ok(contents) = std::fs::read_to_string(&self.path) {
                    if contents.contains("digiOS Model Placeholder") {
                        return Err(anyhow::anyhow!(
                            "Placeholder model file detected: {:?}\n\
                            This is not a real model. Please:\n\
                            1. Select a real model during setup, or\n\
                            2. Download a model, or\n\
                            3. Start LM Studio/Ollama with a loaded model",
                            self.path
                        ));
                    }
                }
            }
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
        
        // Use LM Studio if available
        if let Some(ref client) = self.lm_studio_client {
            info!("Calling LM Studio for inference");
            match client.generate(prompt).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    warn!("LM Studio inference failed: {}. Model may not be loaded in LM Studio.", e);
                    return Err(anyhow::anyhow!(
                        "LM Studio inference failed. Make sure:\n\
                        1. LM Studio is running\n\
                        2. A model is loaded in LM Studio\n\
                        3. Local server is enabled (Settings > Local Server)\n\
                        Error: {}", e
                    ));
                }
            }
        }
        
        // Use Hugging Face if available
        if let Some(ref client) = self.huggingface_client {
            info!("Calling Hugging Face model for inference");
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
        // Recreate clients for clone (simplified - just recreate based on path)
        let path_str = self.path.to_string_lossy();
        let (ollama_client, python_ollama_client, lm_studio_client, huggingface_client, model_name) = 
            if path_str.starts_with("ollama://") {
                let model_name_raw = path_str.strip_prefix("ollama://").unwrap().to_string();
                let full_name = format!("ollama:{}", model_name_raw);
                let ollama_client = OllamaClient::new(model_name_raw.clone());
                let python_client = PythonOllamaClient::new(model_name_raw.clone());
                (Some(ollama_client), Some(python_client), None, None, full_name)
            } else if self.path.exists() && self.path.is_file() {
                let name = self.path.file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("model")
                    .to_string();
                let lm_client = LMStudioClient::new(self.path.clone());
                let hf_client = HuggingFaceClient::new(self.path.clone());
                (None, None, Some(lm_client), Some(hf_client), name)
            } else {
                (None, None, None, None, path_str.to_string())
            };
        
        Self {
            path: self.path.clone(),
            loaded: self.loaded,
            ollama_client,
            python_ollama_client,
            lm_studio_client,
            huggingface_client,
            model_name,
        }
    }
}

