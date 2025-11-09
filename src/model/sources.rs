use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSource {
    pub name: String,
    pub provider: ModelSourceProvider,
    pub description: String,
    pub url: String,
    pub size: ModelSize,
    pub format: ModelFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSourceProvider {
    HuggingFace,
    Ollama,
    LMStudio,
    DirectDownload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSize {
    Tiny,    // < 1GB
    Small,   // 1-3GB
    Medium,  // 3-7GB
    Large,   // 7-15GB
    XLarge,  // 15GB+
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelFormat {
    GGUF,
    Safetensors,
    Pytorch,
    ONNX,
    Other(String),
}

/// Pre-configured model sources for download
pub struct ModelSources;

impl ModelSources {
    /// Get list of recommended models to download
    pub fn get_recommended_models() -> Vec<ModelSource> {
        vec![
            // Small, fast models
            ModelSource {
                name: "Phi-3-mini-4k-instruct".to_string(),
                provider: ModelSourceProvider::HuggingFace,
                description: "Microsoft Phi-3 Mini - Fast, efficient, 3.8B parameters".to_string(),
                url: "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf".to_string(),
                size: ModelSize::Small,
                format: ModelFormat::GGUF,
            },
            ModelSource {
                name: "Qwen2-0.5B-Instruct".to_string(),
                provider: ModelSourceProvider::HuggingFace,
                description: "Qwen2 0.5B - Very small, fast inference".to_string(),
                url: "https://huggingface.co/Qwen/Qwen2-0.5B-Instruct-GGUF".to_string(),
                size: ModelSize::Tiny,
                format: ModelFormat::GGUF,
            },
            // Medium models
            ModelSource {
                name: "Llama-3.2-3B-Instruct".to_string(),
                provider: ModelSourceProvider::HuggingFace,
                description: "Meta Llama 3.2 3B - Balanced performance and quality".to_string(),
                url: "https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF".to_string(),
                size: ModelSize::Medium,
                format: ModelFormat::GGUF,
            },
            ModelSource {
                name: "Mistral-7B-Instruct-v0.3".to_string(),
                provider: ModelSourceProvider::HuggingFace,
                description: "Mistral 7B Instruct - High quality, versatile".to_string(),
                url: "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.3-GGUF".to_string(),
                size: ModelSize::Medium,
                format: ModelFormat::GGUF,
            },
            // Large models
            ModelSource {
                name: "Llama-3.1-8B-Instruct".to_string(),
                provider: ModelSourceProvider::HuggingFace,
                description: "Meta Llama 3.1 8B - High quality, larger context".to_string(),
                url: "https://huggingface.co/bartowski/Llama-3.1-8B-Instruct-GGUF".to_string(),
                size: ModelSize::Large,
                format: ModelFormat::GGUF,
            },
            ModelSource {
                name: "Qwen2.5-7B-Instruct".to_string(),
                provider: ModelSourceProvider::HuggingFace,
                description: "Qwen2.5 7B - Excellent quality, multilingual".to_string(),
                url: "https://huggingface.co/Qwen/Qwen2.5-7B-Instruct-GGUF".to_string(),
                size: ModelSize::Large,
                format: ModelFormat::GGUF,
            },
        ]
    }

    /// Get models filtered by size
    pub fn get_by_size(size: &ModelSize) -> Vec<ModelSource> {
        Self::get_recommended_models()
            .into_iter()
            .filter(|m| matches!(&m.size, size))
            .collect()
    }

    /// Format size for display
    pub fn format_size(size: &ModelSize) -> String {
        match size {
            ModelSize::Tiny => "< 1GB",
            ModelSize::Small => "1-3GB",
            ModelSize::Medium => "3-7GB",
            ModelSize::Large => "7-15GB",
            ModelSize::XLarge => "15GB+",
        }.to_string()
    }

    /// Format provider name
    pub fn format_provider(provider: &ModelSourceProvider) -> String {
        match provider {
            ModelSourceProvider::HuggingFace => "Hugging Face",
            ModelSourceProvider::Ollama => "Ollama",
            ModelSourceProvider::LMStudio => "LM Studio",
            ModelSourceProvider::DirectDownload => "Direct Download",
        }.to_string()
    }
}

