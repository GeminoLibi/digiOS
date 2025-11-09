pub mod manager;
pub mod downloader;
pub mod loader;
pub mod detector;
pub mod sources;

pub use manager::ModelManager;
pub use detector::{ModelDetector, DetectedModel, ModelProvider};
pub use sources::{ModelSources, ModelSource, ModelSourceProvider};

