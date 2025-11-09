use crate::model::downloader::ModelDownloader;
use crate::model::loader::ModelLoader;
use crate::core::paths;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub size: ModelSize,
    pub url: Option<String>,
    pub local_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSize {
    Small,   // ~1GB
    Medium,  // ~4GB
    Large,   // ~8GB+
}

#[derive(Clone)]
pub struct ModelManager {
    config: ModelConfig,
    loader: Arc<RwLock<Option<ModelLoader>>>,
    downloader: ModelDownloader,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        // Load or create config
        let config_path = paths::get_model_config_path();
        let config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let mut config: ModelConfig = serde_json::from_str(&content)
                .map_err(|e| anyhow::anyhow!("Failed to parse model.json: {} (content: {})", e, content.trim()))?;
            // Ensure local_path is correct for current platform
            config.local_path = paths::get_models_dir();
            config
        } else {
            // Default config
            ModelConfig {
                name: "digios-default".to_string(),
                size: ModelSize::Medium,
                url: None,
                local_path: paths::get_models_dir(),
            }
        };

        // Ensure model directory exists
        std::fs::create_dir_all(&config.local_path)?;

        Ok(Self {
            config,
            loader: Arc::new(RwLock::new(None)),
            downloader: ModelDownloader::new(),
        })
    }

    pub async fn model_exists(&self) -> Result<bool> {
        // Check for model file with config name
        let model_file = self.config.local_path.join(&self.config.name);
        if model_file.exists() {
            return Ok(true);
        }
        
        // Also check for common placeholder names
        let placeholder_names = [
            "digios-default",
            "digios-default-small",
            "digios-default-large",
        ];
        
        for name in &placeholder_names {
            if self.config.local_path.join(name).exists() {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    pub async fn download_model(&self) -> Result<()> {
        info!("Downloading model: {}", self.config.name);
        
        // Determine download URL based on size
        let url = match self.config.url {
            Some(ref u) => u.clone(),
            None => self.get_default_url().await?,
        };

        self.downloader.download(&url, &self.config.local_path).await?;
        info!("Model download complete");
        Ok(())
    }

    async fn get_default_url(&self) -> Result<String> {
        // TODO: Implement model URL selection based on size
        // For now, return placeholder
        match self.config.size {
            ModelSize::Small => Ok("https://example.com/models/small.bin".to_string()),
            ModelSize::Medium => Ok("https://example.com/models/medium.bin".to_string()),
            ModelSize::Large => Ok("https://example.com/models/large.bin".to_string()),
        }
    }

    pub async fn load_model(&self) -> Result<()> {
        info!("Loading model: {}", self.config.name);
        
        // Check if this is an Ollama model
        let model_path = if self.config.name.starts_with("ollama:") {
            // Ollama model - use special path format
            PathBuf::from(format!("ollama://{}", self.config.name.strip_prefix("ollama:").unwrap()))
        } else {
            // Try config name first
            let mut model_path = self.config.local_path.join(&self.config.name);
            
            // If not found, try placeholder names
            if !model_path.exists() {
                let placeholder_names = [
                    "digios-default",
                    "digios-default-small", 
                    "digios-default-large",
                ];
                
                let mut found = false;
                for name in &placeholder_names {
                    let test_path = self.config.local_path.join(name);
                    if test_path.exists() {
                        model_path = test_path;
                        found = true;
                        info!("Found model at: {:?}", model_path);
                        break;
                    }
                }
                
                if !found {
                    return Err(anyhow::anyhow!(
                        "Model file not found: {:?}\n\
                        Searched for: {} and placeholder names\n\
                        Directory contents: {:?}",
                        model_path,
                        self.config.name,
                        std::fs::read_dir(&self.config.local_path)
                            .map(|dir| dir.map(|e| e.unwrap().file_name().to_string_lossy().to_string()).collect::<Vec<_>>())
                            .unwrap_or_default()
                    ));
                }
            }
            
            model_path
        };

        let mut loader = ModelLoader::new(&model_path).await?;
        loader.load().await?;
        *self.loader.write().await = Some(loader);
        
        info!("Model loaded successfully: {}", self.config.name);
        Ok(())
    }

    pub async fn get_model(&self) -> Option<ModelLoader> {
        self.loader.read().await.clone()
    }

    pub fn get_config(&self) -> &ModelConfig {
        &self.config
    }
}

