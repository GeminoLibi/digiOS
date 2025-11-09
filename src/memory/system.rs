use crate::core::paths;
use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

pub struct MemorySystem {
    path: PathBuf,
}

impl MemorySystem {
    pub async fn new(path: &str) -> Result<Self> {
        // Use provided path or default
        let memory_path = if path.is_empty() || path == "aios_memory" {
            paths::get_memory_dir()
        } else {
            PathBuf::from(path)
        };
        
        info!("Initializing Memory System at: {:?}", memory_path);
        std::fs::create_dir_all(&memory_path)?;
        
        Ok(Self { path: memory_path })
    }
}

