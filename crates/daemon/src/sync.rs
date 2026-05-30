//! e-mail synchronization logic

use crate::state::DaemonState;
use threadbare_core::Result;
use tracing::info;

/// e-mail synchronization manager
pub struct SyncManager {
    state: DaemonState,
}

impl SyncManager {
    /// create a new sync manager
    pub fn new(state: DaemonState) -> Self {
        Self { state }
    }

    /// start the sync loop
    pub async fn start_sync_loop(&self) -> Result<()> {
        let sync_interval = self.state.config().daemon.sync_interval;
        
        loop {
            info!("Starting sync cycle");
            
            if let Err(e) = self.sync_all_accounts().await {
                tracing::error!("Sync error: {}", e);
            }

            // wait for the next sync interval
            tokio::time::sleep(tokio::time::Duration::from_secs(sync_interval)).await;
        }
    }

    /// sync all accounts
    async fn sync_all_accounts(&self) -> Result<()> {
        info!("Syncing all accounts");
        Ok(())
    }
}

