use aios::core::aios::aiOS;
use anyhow::Result;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("aios=debug,info")
        .init();

    info!("Starting aiOS (Rust)");

    // Create and start aiOS
    let mut aios = aiOS::new().await?;
    
    info!("aiOS initialized, starting service...");
    aios.start().await?;

    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;
    
    info!("Shutting down aiOS...");
    aios.shutdown().await?;

    Ok(())
}
