use anyhow::Result;
use tracing::{info, warn};
use std::io::{self, Write};

/// Setup Wizard - Runs on first boot
pub struct SetupWizard;

impl SetupWizard {
    pub async fn run() -> Result<()> {
        info!("=== digiOS First Boot Setup ===");
        
        println!("\nWelcome to digiOS - AI Native Operating System");
        println!("This setup will configure your system.\n");
        
        // Network configuration
        Self::configure_network().await?;
        
        // Model selection
        Self::select_model().await?;
        
        // Human interaction preferences
        Self::configure_human_interface().await?;
        
        // System preferences
        Self::configure_system().await?;
        
        println!("\nSetup complete! System will now boot...\n");
        Ok(())
    }

    async fn configure_network() -> Result<()> {
        println!("Network Configuration:");
        println!("1. Auto-detect (DHCP)");
        println!("2. Manual configuration");
        print!("Select option [1]: ");
        io::stdout().flush()?;
        
        // For now, auto-detect
        info!("Network: Auto-detect (DHCP)");
        Ok(())
    }

    async fn select_model() -> Result<()> {
        use crate::model::detector::{ModelDetector, ModelProvider};
        
        println!("\nModel Selection:");
        
        // Detect installed models
        let detector = ModelDetector::new();
        let detected_models = detector.detect_models().await?;
        
        if !detected_models.is_empty() {
            println!("\nFound {} installed model(s) on your system:", detected_models.len());
            println!();
            
            for (i, model) in detected_models.iter().enumerate() {
                let size_str = ModelDetector::format_size(model.size);
                let provider_str = match &model.provider {
                    ModelProvider::Ollama => "Ollama",
                    ModelProvider::LMStudio => "LM Studio",
                    ModelProvider::HuggingFace => "Hugging Face",
                    ModelProvider::LocalFile => "Local File",
                    ModelProvider::OpenAI => "OpenAI API",
                    ModelProvider::Anthropic => "Anthropic API",
                    ModelProvider::Other(name) => name,
                };
                
                println!("  {}. {} ({}) - {}", 
                    i + 1, 
                    model.name, 
                    provider_str,
                    size_str
                );
                println!("     {}", model.description);
                println!("     Path: {:?}", model.path);
                println!();
            }
            
            println!("Options:");
            println!("  [1-{}] - Use detected model", detected_models.len());
            println!("  d - Download new model (placeholder)");
            println!("  s - Skip model setup for now");
            print!("\nSelect option: ");
            io::stdout().flush()?;
            
            // For now, default to first detected model
            // TODO: Read user input
            if let Some(first_model) = detected_models.first() {
                info!("Selected model: {} ({:?})", first_model.name, first_model.path);
                println!("Using: {}", first_model.name);
                
                // Save selection to config
                Self::save_model_selection(&first_model).await?;
            } else {
                warn!("No model selected");
            }
        } else {
            println!("\nNo AI models detected on your system.");
            println!("\nAvailable models to download:");
            println!();
            
            use crate::model::sources::ModelSources;
            let recommended = ModelSources::get_recommended_models();
            
            for (i, model) in recommended.iter().enumerate() {
                let size_str = ModelSources::format_size(&model.size);
                let provider_str = ModelSources::format_provider(&model.provider);
                
                println!("  {}. {} ({}) - {}", 
                    i + 1,
                    model.name,
                    provider_str,
                    size_str
                );
                println!("     {}", model.description);
                println!();
            }
            
            println!("Options:");
            println!("  [1-{}] - Download selected model", recommended.len());
            println!("  s - Skip model setup for now");
            print!("\nSelect option [s]: ");
            io::stdout().flush()?;
            
            // For now, default to skip (TODO: implement user input)
            // If user selects a model, download it
            info!("Model setup skipped - user can download later");
            println!("Skipping model download. You can configure a model later.");
            println!("To download a model, run digiOS again or use the API.");
        }
        
        Ok(())
    }
    
    async fn save_model_selection(model: &crate::model::detector::DetectedModel) -> Result<()> {
        use crate::core::paths;
        use crate::model::manager::{ModelConfig, ModelSize};
        use serde_json;
        use std::fs;
        
        let config_path = paths::get_model_config_path();
        let config_dir = config_path.parent().unwrap();
        fs::create_dir_all(config_dir)?;
        
        // Determine size based on file size if available
        let size = if let Some(file_size) = model.size {
            if file_size < 2_000_000_000 {
                ModelSize::Small
            } else if file_size < 8_000_000_000 {
                ModelSize::Medium
            } else {
                ModelSize::Large
            }
        } else {
            ModelSize::Medium
        };
        
        let config = ModelConfig {
            name: model.name.clone(),
            size,
            url: None,
            local_path: model.path.parent().unwrap_or(&paths::get_models_dir()).to_path_buf(),
        };
        
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(&config_path, json)?;
        
        info!("Saved model selection to: {:?}", config_path);
        Ok(())
    }

    async fn configure_human_interface() -> Result<()> {
        println!("\nHuman Interaction:");
        println!("1. Allow human interaction");
        println!("2. AI-only mode");
        print!("Select mode [1]: ");
        io::stdout().flush()?;
        
        info!("Human interface: Enabled");
        Ok(())
    }

    async fn configure_system() -> Result<()> {
        println!("\nSystem Configuration:");
        println!("- Self-improvement: Enabled");
        println!("- Recursive building: Enabled");
        println!("- Safety mode: Enabled");
        
        info!("System configured");
        Ok(())
    }
}

