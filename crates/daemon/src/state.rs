//! daemon state management

use std::sync::Arc;
use sqlx::sqlite::SqlitePool;
use parking_lot::RwLock;
use threadbare_core::config::Config;
use threadbare_core::Result;

/// shared daemon state
#[derive(Clone)]
pub struct DaemonState {
    /// database connection pool
    pub db: SqlitePool,
    
    /// configuration
    pub config: Arc<RwLock<Config>>,
}

impl DaemonState {
    /// create a new daemon state
    pub async fn new(config: Config) -> Result<Self> {
        // create database connection pool
        let db = SqlitePool::connect(&format!(
            "sqlite://{}",
            config.daemon.database_path.display()
        ))
        .await?;

        // initialize database schema
        threadbare_core::db::init(&db).await?;

        Ok(Self {
            db,
            config: Arc::new(RwLock::new(config)),
        })
    }

    /// get a read lock on the config
    pub fn config(&self) -> parking_lot::RwLockReadGuard<'_, Config> {
        self.config.read()
    }

    /// get a write lock on the config
    pub fn config_mut(&self) -> parking_lot::RwLockWriteGuard<'_, Config> {
        self.config.write()
    }
}

