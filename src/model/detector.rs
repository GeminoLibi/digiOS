use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedModel {
    pub name: String,
    pub provider: ModelProvider,
    pub path: PathBuf,
    pub size: Option<u64>, // in bytes
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelProvider {
    Ollama,
    LMStudio,
    HuggingFace,
    LocalFile,
    OpenAI, // For API-based models
    Anthropic, // For Claude API
    Other(String),
}

/// Detects AI models installed on the host system
pub struct ModelDetector;

impl ModelDetector {
    pub fn new() -> Self {
        Self
    }

    /// Scan the system for installed AI models
    pub async fn detect_models(&self) -> Result<Vec<DetectedModel>> {
        let mut models = Vec::new();

        info!("Scanning system for installed AI models...");

        // Check Ollama
        if let Ok(ollama_models) = self.detect_ollama().await {
            models.extend(ollama_models);
        }

        // Check LM Studio
        if let Ok(lm_models) = self.detect_lm_studio().await {
            models.extend(lm_models);
        }

        // Check Hugging Face cache
        if let Ok(hf_models) = self.detect_huggingface().await {
            models.extend(hf_models);
        }

        // Check common local model directories
        if let Ok(local_models) = self.detect_local_models().await {
            models.extend(local_models);
        }

        info!("Found {} model(s) on system", models.len());
        Ok(models)
    }

    async fn detect_ollama(&self) -> Result<Vec<DetectedModel>> {
        let mut models = Vec::new();

        // Check if ollama is installed
        let ollama_check = Command::new("ollama")
            .arg("list")
            .output();

        if let Ok(output) = ollama_check {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                
                // Parse ollama list output
                // Format: NAME    SIZE    MODIFIED
                for line in stdout.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if !parts.is_empty() {
                        let name = parts[0].to_string();
                        models.push(DetectedModel {
                            name: format!("ollama:{}", name),
                            provider: ModelProvider::Ollama,
                            path: PathBuf::from(format!("ollama://{}", name)),
                            size: None,
                            description: format!("Ollama model: {}", name),
                        });
                    }
                }
            }
        }

        // Also check Ollama model directory
        let ollama_dir = if cfg!(windows) {
            PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default())
                .join("AppData").join("Local").join("Programs").join("Ollama")
        } else if cfg!(target_os = "macos") {
            PathBuf::from(std::env::var("HOME").unwrap_or_default())
                .join(".ollama")
        } else {
            PathBuf::from(std::env::var("HOME").unwrap_or_default())
                .join(".ollama")
        };

        if ollama_dir.exists() {
            info!("Found Ollama directory: {:?}", ollama_dir);
        }

        Ok(models)
    }

    async fn detect_lm_studio(&self) -> Result<Vec<DetectedModel>> {
        let mut models = Vec::new();

        // LM Studio typically stores models in:
        // Windows: %APPDATA%\Local\LM Studio\models
        // macOS: ~/Library/Application Support/LM Studio/models
        // Linux: ~/.local/share/lm-studio/models

        let lm_dir = if cfg!(windows) {
            PathBuf::from(std::env::var("APPDATA").unwrap_or_default())
                .join("Local").join("LM Studio").join("models")
        } else if cfg!(target_os = "macos") {
            PathBuf::from(std::env::var("HOME").unwrap_or_default())
                .join("Library").join("Application Support")
                .join("LM Studio").join("models")
        } else {
            PathBuf::from(std::env::var("HOME").unwrap_or_default())
                .join(".local").join("share").join("lm-studio").join("models")
        };

        if lm_dir.exists() {
            info!("Found LM Studio directory: {:?}", lm_dir);
            
            // Scan for model files (common extensions: .gguf, .bin, .safetensors)
            if let Ok(entries) = std::fs::read_dir(&lm_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let ext = path.extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("");
                        
                        if matches!(ext, "gguf" | "bin" | "safetensors" | "pt" | "pth") {
                            let name = path.file_stem()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string();
                            
                            let size = path.metadata().ok().map(|m| m.len());
                            
                            models.push(DetectedModel {
                                name: format!("lmstudio:{}", name),
                                provider: ModelProvider::LMStudio,
                                path: path.clone(),
                                size,
                                description: format!("LM Studio model: {} ({})", name, ext),
                            });
                        }
                    }
                }
            }
        }

        Ok(models)
    }

    async fn detect_huggingface(&self) -> Result<Vec<DetectedModel>> {
        let mut models = Vec::new();

        // Hugging Face cache location
        let hf_dir = if cfg!(windows) {
            PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default())
                .join(".cache").join("huggingface")
        } else {
            PathBuf::from(std::env::var("HOME").unwrap_or_default())
                .join(".cache").join("huggingface")
        };

        if hf_dir.exists() {
            info!("Found Hugging Face cache: {:?}", hf_dir);
            
            // Scan hub directory for models
            let hub_dir = hf_dir.join("hub");
            if hub_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&hub_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            // Look for model files
                            if let Ok(model_files) = std::fs::read_dir(&path) {
                                let mut has_model = false;
                                for file in model_files.flatten() {
                                    let file_path = file.path();
                                    if file_path.is_file() {
                                        let ext = file_path.extension()
                                            .and_then(|e| e.to_str())
                                            .unwrap_or("");
                                        
                                        if matches!(ext, "bin" | "safetensors" | "pt" | "pth" | "onnx") {
                                            has_model = true;
                                            break;
                                        }
                                    }
                                }
                                
                                if has_model {
                                    let name = path.file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("unknown")
                                        .to_string();
                                    
                                    models.push(DetectedModel {
                                        name: format!("huggingface:{}", name),
                                        provider: ModelProvider::HuggingFace,
                                        path: path.clone(),
                                        size: None, // Would need to calculate
                                        description: format!("Hugging Face model: {}", name),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(models)
    }

    async fn detect_local_models(&self) -> Result<Vec<DetectedModel>> {
        let mut models = Vec::new();

        // Check common model directories
        let common_dirs = vec![
            // digiOS models directory
            PathBuf::from("E:\\digiOS\\var\\lib\\digios\\models"),
            PathBuf::from("var\\lib\\digios\\models"),
            PathBuf::from("/var/lib/digios/models"),
            // Common user directories
            if cfg!(windows) {
                PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default())
                    .join("models")
            } else {
                PathBuf::from(std::env::var("HOME").unwrap_or_default())
                    .join("models")
            },
            PathBuf::from("models"),
        ];

        for dir in common_dirs {
            if dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            let ext = path.extension()
                                .and_then(|e| e.to_str())
                                .unwrap_or("");
                            
                            if matches!(ext, "gguf" | "bin" | "safetensors" | "pt" | "pth" | "onnx" | "model") {
                                let name = path.file_stem()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("unknown")
                                    .to_string();
                                
                                let size = path.metadata().ok().map(|m| m.len());
                                
                                models.push(DetectedModel {
                                    name: format!("local:{}", name),
                                    provider: ModelProvider::LocalFile,
                                    path: path.clone(),
                                    size,
                                    description: format!("Local model file: {} ({})", name, ext),
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(models)
    }

    /// Format model size for display
    pub fn format_size(size_bytes: Option<u64>) -> String {
        if let Some(size) = size_bytes {
            if size < 1024 {
                format!("{} B", size)
            } else if size < 1024 * 1024 {
                format!("{:.2} KB", size as f64 / 1024.0)
            } else if size < 1024 * 1024 * 1024 {
                format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
            } else {
                format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
            }
        } else {
            "Unknown".to_string()
        }
    }
}

