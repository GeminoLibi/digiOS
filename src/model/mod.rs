pub mod manager;
pub mod downloader;
pub mod loader;
pub mod detector;
pub mod sources;
pub mod ollama;
pub mod python_ollama;
pub mod lm_studio;
pub mod huggingface;

pub use manager::ModelManager;
pub use detector::{ModelDetector, DetectedModel, ModelProvider};
pub use sources::{ModelSources, ModelSource, ModelSourceProvider};

