use crate::model::downloader::ModelDownloader;
use crate::model::loader::ModelLoader;
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

pub struct ModelManager {
    config: ModelConfig,
    loader: Arc<RwLock<Option<ModelLoader>>>,
    downloader: ModelDownloader,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        // Load or create config
        let config_path = PathBuf::from("/etc/digios/model.json");
        let config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            serde_json::from_str(&content)?
        } else {
            // Default config
            ModelConfig {
                name: "digios-default".to_string(),
                size: ModelSize::Medium,
                url: None,
                local_path: PathBuf::from("/var/lib/digios/models"),
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
        let model_file = self.config.local_path.join(&self.config.name);
        Ok(model_file.exists())
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
        
        let model_path = self.config.local_path.join(&self.config.name);
        if !model_path.exists() {
            return Err(anyhow::anyhow!("Model file not found: {:?}", model_path));
        }

        let loader = ModelLoader::new(&model_path).await?;
        *self.loader.write().await = Some(loader);
        
        info!("Model loaded successfully");
        Ok(())
    }

    pub async fn get_model(&self) -> Option<ModelLoader> {
        self.loader.read().await.clone()
    }

    pub fn get_config(&self) -> &ModelConfig {
        &self.config
    }
}

