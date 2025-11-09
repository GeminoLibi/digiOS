use crate::core::aios::aiOS;
use crate::model::ModelManager;
use crate::self_improve::SelfImprovementEngine;
use anyhow::Result;
use tracing::{info, error};

/// Init System - First process that runs on boot
/// Responsible for system initialization and starting core services
pub struct InitSystem {
    aios: Option<aiOS>,
    model_manager: Option<ModelManager>,
    self_improve: Option<SelfImprovementEngine>,
}

impl InitSystem {
    pub async fn new() -> Result<Self> {
        info!("digiOS Init System starting...");
        Ok(Self {
            aios: None,
            model_manager: None,
            self_improve: None,
        })
    }

    /// Main boot sequence
    pub async fn boot(&mut self) -> Result<()> {
        info!("=== digiOS Boot Sequence ===");
        
        // Phase 1: System initialization
        info!("Phase 1: System Initialization");
        self.initialize_system().await?;
        
        // Phase 2: Model setup
        info!("Phase 2: Model Setup");
        self.setup_model().await?;
        
        // Phase 3: Start core services
        info!("Phase 3: Starting Core Services");
        self.start_core_services().await?;
        
        // Phase 4: Initialize self-improvement
        info!("Phase 4: Initializing Self-Improvement Engine");
        self.initialize_self_improvement().await?;
        
        // Phase 5: Begin recursive building
        info!("Phase 5: Beginning Recursive Self-Building");
        self.begin_recursive_building().await?;
        
        info!("=== digiOS Boot Complete ===");
        Ok(())
    }

    async fn initialize_system(&mut self) -> Result<()> {
        info!("Initializing digiOS core system...");
        
        // Create aiOS instance
        let mut aios = aiOS::new().await?;
        aios.start().await?;
        
        self.aios = Some(aios);
        info!("Core system initialized");
        Ok(())
    }

    async fn setup_model(&mut self) -> Result<()> {
        info!("Setting up AI model...");
        
        let model_manager = ModelManager::new().await?;
        
        // Check if model exists, download if not
        if !model_manager.model_exists().await? {
            info!("Model not found. Downloading...");
            model_manager.download_model().await?;
        } else {
            info!("Model found. Loading...");
        }
        
        // Load model
        model_manager.load_model().await?;
        info!("Model loaded and ready");
        
        self.model_manager = Some(model_manager);
        Ok(())
    }

    async fn start_core_services(&mut self) -> Result<()> {
        info!("Starting core services...");
        
        // Services are started by aiOS
        // Additional services can be started here
        
        info!("Core services running");
        Ok(())
    }

    async fn initialize_self_improvement(&mut self) -> Result<()> {
        info!("Initializing self-improvement engine...");
        
        let aios = self.aios.as_ref().unwrap();
        let model_manager = self.model_manager.as_ref().unwrap();
        
        let self_improve = SelfImprovementEngine::new(
            aios.clone(),
            model_manager.clone(),
        ).await?;
        
        self.self_improve = Some(self_improve);
        info!("Self-improvement engine initialized");
        Ok(())
    }

    async fn begin_recursive_building(&mut self) -> Result<()> {
        info!("Beginning recursive self-building process...");
        
        if let Some(ref mut self_improve) = self.self_improve {
            // Start the recursive building loop
            self_improve.start_recursive_building().await?;
        }
        
        Ok(())
    }

    /// Check if this is first boot (setup needed)
    pub async fn is_first_boot(&self) -> bool {
        // Check for setup flag or config file
        !std::path::Path::new("/etc/digios/setup_complete").exists()
    }

    /// Run first-boot setup
    pub async fn run_setup(&mut self) -> Result<()> {
        info!("Running first-boot setup...");
        
        // Create setup directory
        std::fs::create_dir_all("/etc/digios")?;
        
        // Run setup wizard
        crate::boot::setup::SetupWizard::run().await?;
        
        // Mark setup as complete
        std::fs::write("/etc/digios/setup_complete", "1")?;
        
        info!("Setup complete");
        Ok(())
    }
}

