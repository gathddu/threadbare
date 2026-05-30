//! threadbare daemon binary
//! 
//! runs in the background, syncs e-mails and provides IPC interface.

use tracing_subscriber;
use threadbare_core::config::Config;
use threadbare_daemon::{DaemonServer, DaemonState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // load or create config
    let config = Config::default();
    
    tracing::info!("Starting threadbare daemon v{}", threadbare_daemon::VERSION);
    tracing::info!("Config dir: {}", config.app.config_dir.display());
    tracing::info!("Data dir: {}", config.app.data_dir.display());

    // create daemon state
    let state = DaemonState::new(config.clone()).await?;
    
    // create IPC server
    let server = DaemonServer::new(
        config.daemon.ipc_socket.clone(),
        state.clone(),
    );

    // start sync manager in background
    let sync_state = state.clone();
    tokio::spawn(async move {
        use threadbare_daemon::sync::SyncManager;
        let sync_manager = SyncManager::new(sync_state);
        if let Err(e) = sync_manager.start_sync_loop().await {
            tracing::error!("Sync manager error: {}", e);
        }
    });

    // start IPC server (blocking)
    server.start().await?;

    Ok(())
}

