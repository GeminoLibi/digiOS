use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    pub memory: MemoryConfig,
    pub system: SystemConfig,
    pub features: FeaturesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub path: String,
    pub max_history: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub safety_mode: bool,
    pub enable_kernel_ops: bool,
    pub max_concurrent_tasks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub vision: bool,
    pub ocr: bool,
    pub object_detection: bool,
    pub event_monitoring: bool,
    pub memory_persistence: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api: ApiConfig {
                port: 8765,
                host: "0.0.0.0".to_string(),
            },
            memory: MemoryConfig {
                path: "aios_memory".to_string(),
                max_history: 10000,
            },
            system: SystemConfig {
                safety_mode: true,
                enable_kernel_ops: false,
                max_concurrent_tasks: 10,
            },
            features: FeaturesConfig {
                vision: true,
                ocr: true,
                object_detection: true,
                event_monitoring: true,
                memory_persistence: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> Self {
        // Try to load from config.json, fallback to default
        if let Ok(content) = std::fs::read_to_string("config.json") {
            if let Ok(config) = serde_json::from_str::<Config>(&content) {
                return config;
            }
        }
        Config::default()
    }
}

