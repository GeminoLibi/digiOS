use crate::action::ActionEngine;
use crate::api::server::AppState;
use crate::event::EventSystem;
use crate::memory::MemorySystem;
use crate::state::StateManager;
use crate::task::TaskPlanner;
use crate::vision::VisionSystem;
use crate::core::config::Config;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};


#[derive(Clone)]
pub struct aiOS {
    config: Config,
    action_engine: Arc<ActionEngine>,
    vision: Arc<VisionSystem>,
    state_manager: Arc<StateManager>,
    task_planner: Arc<TaskPlanner>,
    event_system: Arc<EventSystem>,
    memory: Arc<MemorySystem>,
    running: Arc<RwLock<bool>>,
}

impl aiOS {
    pub async fn new() -> Result<Self> {
        let config = Config::load();
        
        info!("Initializing aiOS with config: {:?}", config);

        // Initialize components
        let action_engine = Arc::new(ActionEngine::new().await?);
        let vision = Arc::new(VisionSystem::new().await?);
        let state_manager = Arc::new(StateManager::new().await?);
        let memory = Arc::new(MemorySystem::new(&config.memory.path).await?);
        
        let task_planner = Arc::new(TaskPlanner::new(
            action_engine.clone(),
            vision.clone(),
            state_manager.clone(),
        ));

        let event_system = Arc::new(EventSystem::new().await?);

        Ok(Self {
            config,
            action_engine,
            vision,
            state_manager,
            task_planner,
            event_system,
            memory,
            running: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting aiOS service...");

        // Start background tasks
        self.start_background_tasks().await?;

        // Create app state for API server
        let app_state = crate::api::server::AppState {
            action_engine: self.action_engine.clone(),
            vision: self.vision.clone(),
            state_manager: self.state_manager.clone(),
            task_planner: self.task_planner.clone(),
            memory: self.memory.clone(),
        };

        // Build router
        let router = axum::Router::new()
            .route("/api/status", axum::routing::get(crate::api::server::handle_status))
            .route("/api/capabilities", axum::routing::get(crate::api::server::handle_capabilities))
            .route("/api/action", axum::routing::post(crate::api::server::handle_execute_action))
            .route("/api/vision/screenshot", axum::routing::get(crate::api::server::handle_screenshot))
            .with_state(app_state.clone());

        // Start server in background
        let addr = format!("{}:{}", self.config.api.host, self.config.api.port);
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            info!("API Server listening on {}", addr);
            axum::serve(listener, router).await.unwrap();
        });

        *self.running.write().await = true;

        info!("aiOS service started on {}:{}", self.config.api.host, self.config.api.port);
        Ok(())
    }

    async fn start_background_tasks(&self) -> Result<()> {
        // Start state monitoring
        let state_manager = self.state_manager.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                if let Err(e) = state_manager.update().await {
                    error!("State update error: {}", e);
                }
            }
        });

        // Start event processing
        let event_system = self.event_system.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = event_system.process_events().await {
                    error!("Event processing error: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down aiOS...");
        *self.running.write().await = false;
        info!("aiOS shutdown complete");
        Ok(())
    }

    pub fn get_capabilities(&self) -> serde_json::Value {
        serde_json::json!({
            "actions": self.action_engine.get_available_actions(),
            "vision": self.vision.get_capabilities(),
            "system": {
                "platform": std::env::consts::OS,
                "kernel_ops_available": self.config.system.enable_kernel_ops
            }
        })
    }
}

