use crate::core::aios::aiOS;
use crate::core::paths;
use crate::model::ModelManager;
use crate::self_improve::SelfImprovementEngine;
use crate::interaction::{InteractionManager, ToolManager};
use anyhow::Result;
use std::sync::Arc;
use tracing::{info, error};

/// Init System - First process that runs on boot
/// Responsible for system initialization and starting core services
pub struct InitSystem {
    aios: Option<Arc<aiOS>>,
    model_manager: Option<Arc<ModelManager>>,
    self_improve: Option<SelfImprovementEngine>,
    interaction: Option<InteractionManager>,
    tool_manager: Option<ToolManager>,
}

impl InitSystem {
    pub async fn new() -> Result<Self> {
        info!("digiOS Init System starting...");
        Ok(Self {
            aios: None,
            model_manager: None,
            self_improve: None,
            interaction: None,
            tool_manager: None,
        })
    }

    /// Main boot sequence
    pub async fn boot(&mut self) -> Result<()> {
        info!("=== digiOS Boot Sequence ===");
        
        // Phase 1: System initialization
        info!("Phase 1: System Initialization");
        self.initialize_system().await?;
        
        // Phase 2: Tool setup
        info!("Phase 2: Tool Setup");
        self.setup_tools().await?;
        
        // Phase 3: Model setup
        info!("Phase 3: Model Setup");
        self.setup_model().await?;
        
        // Phase 4: Start core services
        info!("Phase 4: Starting Core Services");
        self.start_core_services().await?;
        
        // Phase 5: Initialize interaction
        info!("Phase 5: Initializing Interaction System");
        self.initialize_interaction().await?;
        
        // Phase 6: Initialize self-improvement
        info!("Phase 6: Initializing Self-Improvement Engine");
        self.initialize_self_improvement().await?;
        
        // Phase 7: Begin recursive building
        info!("Phase 7: Beginning Recursive Self-Building");
        self.begin_recursive_building().await?;
        
        info!("=== digiOS Boot Complete ===");
        Ok(())
    }

    async fn initialize_system(&mut self) -> Result<()> {
        info!("Initializing digiOS core system...");
        
        // Create aiOS instance
        let mut aios = aiOS::new().await?;
        aios.start().await?;
        
        self.aios = Some(Arc::new(aios));
        info!("Core system initialized");
        Ok(())
    }

    async fn setup_tools(&mut self) -> Result<()> {
        info!("Setting up tools and compatibility layer...");
        
        let mut tool_manager = ToolManager::new();
        
        // Ensure all required tools are available
        tool_manager.ensure_all_required().await?;
        
        self.tool_manager = Some(tool_manager);
        info!("Tools setup complete");
        Ok(())
    }

    async fn initialize_interaction(&mut self) -> Result<()> {
        info!("Initializing interaction system...");
        
        let interaction = InteractionManager::new().await?;
        
        // Check if human interface should be enabled
        if std::env::var("DIGIOS_HUMAN_INTERFACE").is_ok() {
            interaction.start_terminal().await?;
        }
        
        self.interaction = Some(interaction);
        info!("Interaction system initialized");
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
        
        self.model_manager = Some(Arc::new(model_manager));
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
        
        let aios = self.aios.as_ref().unwrap().clone();
        let model_manager = self.model_manager.as_ref().unwrap().clone();
        
        let self_improve = SelfImprovementEngine::new(
            aios,
            model_manager,
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
        !paths::get_setup_complete_path().exists()
    }

    /// Run first-boot setup
    pub async fn run_setup(&mut self) -> Result<()> {
        info!("Running first-boot setup...");
        
        // Create setup directory
        std::fs::create_dir_all(paths::get_config_dir())?;
        
        // Run setup wizard
        crate::boot::setup::SetupWizard::run().await?;
        
        // Mark setup as complete
        std::fs::write(paths::get_setup_complete_path(), "1")?;
        
        info!("Setup complete");
        Ok(())
    }
}

