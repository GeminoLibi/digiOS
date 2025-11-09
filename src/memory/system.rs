use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub struct MemorySystem {
    path: PathBuf,
}

impl MemorySystem {
    pub async fn new(path: &str) -> Result<Self> {
        info!("Initializing Memory System at: {}", path);
        let path = PathBuf::from(path);
        std::fs::create_dir_all(&path)?;
        
        Ok(Self { path })
    }
}

