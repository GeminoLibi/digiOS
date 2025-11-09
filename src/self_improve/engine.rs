use crate::core::aios::aiOS;
use crate::model::ModelManager;
use crate::self_improve::codegen::CodeGenerator;
use crate::self_improve::evaluator::SystemEvaluator;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

/// Self-Improvement Engine - Recursively builds and improves the system
pub struct SelfImprovementEngine {
    aios: Arc<aiOS>,
    model_manager: Arc<ModelManager>,
    codegen: Arc<CodeGenerator>,
    evaluator: Arc<SystemEvaluator>,
    running: Arc<RwLock<bool>>,
    iteration: Arc<RwLock<u64>>,
}

impl SelfImprovementEngine {
    pub async fn new(
        aios: Arc<aiOS>,
        model_manager: Arc<ModelManager>,
    ) -> Result<Self> {
        info!("Creating Self-Improvement Engine");
        
        let codegen = Arc::new(CodeGenerator::new(model_manager.clone()).await?);
        let evaluator = Arc::new(SystemEvaluator::new());
        
        Ok(Self {
            aios,
            model_manager,
            codegen,
            evaluator,
            running: Arc::new(RwLock::new(false)),
            iteration: Arc::new(RwLock::new(0)),
        })
    }

    /// Start the recursive building process
    pub async fn start_recursive_building(&self) -> Result<()> {
        info!("Starting recursive self-building process");
        *self.running.write().await = true;
        
        // Start the improvement loop
        let running = self.running.clone();
        let iteration = self.iteration.clone();
        let codegen = self.codegen.clone();
        let evaluator = self.evaluator.clone();
        let aios = self.aios.clone();
        
        tokio::spawn(async move {
            loop {
                // Check if we should continue
                if !*running.read().await {
                    break;
                }
                
                let iter = {
                    let mut iter = iteration.write().await;
                    *iter += 1;
                    *iter
                };
                
                info!("Self-improvement iteration {}", iter);
                
                // 1. Evaluate current system
                let evaluation = evaluator.evaluate(&aios).await;
                info!("System evaluation: {:?}", evaluation);
                
                // 2. Identify improvements
                let improvements = evaluator.identify_improvements(&evaluation).await;
                info!("Identified {} potential improvements", improvements.len());
                
                // 3. Generate code for improvements
                for improvement in improvements {
                    match codegen.generate_code(&improvement).await {
                        Ok(code) => {
                            info!("Generated code for: {}", improvement);
                            // TODO: Compile and integrate new code
                        }
                        Err(e) => {
                            error!("Failed to generate code: {}", e);
                        }
                    }
                }
                
                // 4. Wait before next iteration
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });
        
        Ok(())
    }

    pub async fn stop(&self) {
        *self.running.write().await = false;
        info!("Self-improvement engine stopped");
    }

    pub async fn get_iteration(&self) -> u64 {
        *self.iteration.read().await
    }
}

