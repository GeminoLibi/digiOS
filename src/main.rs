use aios::boot::InitSystem;
use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("digios=debug,info")
        .init();

    info!("digiOS - AI Native Operating System");
    info!("Version: 0.1.0");

    // Create init system
    let mut init = InitSystem::new().await?;

    // Check if first boot
    if init.is_first_boot().await {
        info!("First boot detected - running setup");
        init.run_setup().await?;
    }

    // Boot the system
    init.boot().await?;

    // Interaction system is started by init system
    // Human interface will be enabled if DIGIOS_HUMAN_INTERFACE is set

    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;
    
    info!("Shutting down digiOS...");
    // Cleanup will happen automatically

    Ok(())
}
