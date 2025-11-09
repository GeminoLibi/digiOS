use crate::human_interface::TerminalInterface;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

/// Interaction Manager - Handles all forms of interaction
pub struct InteractionManager {
    terminal: Arc<RwLock<Option<TerminalInterface>>>,
    web_enabled: bool,
    voice_enabled: bool,
    auto_tool_install: bool,
}

impl InteractionManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing Interaction Manager");
        
        Ok(Self {
            terminal: Arc::new(RwLock::new(None)),
            web_enabled: false,
            voice_enabled: false,
            auto_tool_install: true, // Auto-install tools by default
        })
    }

    pub async fn start_terminal(&self) -> Result<()> {
        info!("Starting terminal interface");
        let terminal = TerminalInterface::new().await?;
        
        // Clone terminal for background task
        let mut terminal_clone = terminal.clone();
        
        // Start terminal in background task
        tokio::spawn(async move {
            if let Err(e) = terminal_clone.start().await {
                error!("Terminal interface error: {}", e);
            }
        });
        
        // Store reference
        *self.terminal.write().await = Some(terminal);
        Ok(())
    }

    pub async fn enable_web(&mut self) -> Result<()> {
        info!("Enabling web interface");
        self.web_enabled = true;
        // TODO: Start web server
        Ok(())
    }

    pub async fn enable_voice(&mut self) -> Result<()> {
        info!("Enabling voice interface");
        self.voice_enabled = true;
        // TODO: Start voice recognition
        Ok(())
    }

    pub fn is_auto_tool_install(&self) -> bool {
        self.auto_tool_install
    }

    pub async fn handle_interaction(&self, input: &str) -> Result<String> {
        // Route interaction to appropriate interface
        if let Some(ref _terminal) = *self.terminal.read().await {
            // Handle terminal input
            return Ok(format!("Terminal: {}", input));
        }
        
        Ok("No interface available".to_string())
    }
}

