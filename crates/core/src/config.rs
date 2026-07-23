//! configuration management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// application settings
    pub app: AppConfig,
    
    /// daemon settings
    pub daemon: DaemonConfig,
    
    /// GUI settings
    pub gui: GuiConfig,

    /// account settings
    pub accounts: Vec<AccountConfig>,
}

/// application-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// application name
    pub name: String,
    
    /// application version
    pub version: String,
    
    /// data directory path
    pub data_dir: PathBuf,
    
    /// config directory path
    pub config_dir: PathBuf,
    
    /// log level (trace, debug, info, warn, error)
    pub log_level: String,
}

/// daemon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// IPC socket path
    pub ipc_socket: PathBuf,
    
    /// sync interval (seconds)
    pub sync_interval: u64,
    
    /// database path
    pub database_path: PathBuf,
    
    /// max concurrent connections
    pub max_connections: u32,
}

/// GUI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiConfig {
    /// window width
    pub window_width: i32,
    
    /// window height
    pub window_height: i32,
    
    /// theme (light, dark, auto)
    pub theme: String,
    
    /// font size
    pub font_size: i32,
}

/// e-mail account configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    /// display name
    pub name: String,

    /// e-mail address
    pub email: String,

    /// IMAP server hostname
    pub imap_host: String,

    /// IMAP port
    pub imap_port: u16,

    /// SMTP server hostname
    pub smtp_host: String,

    /// SMTP port
    pub smtp_port: u16,

    /// username (same as e-mail?)
    pub username: String,

    /// use TLS
    pub use_tls: bool,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("threadbare");
        
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("threadbare");

        Self {
            app: AppConfig {
                name: "threadbare".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                data_dir: data_dir.clone(),
                config_dir: config_dir.clone(),
                log_level: "info".to_string(),
            },
            daemon: DaemonConfig {
                ipc_socket: config_dir.join("threadbare.sock"),
                sync_interval: 300, // 5 minutes
                database_path: data_dir.join("emails.db"),
                max_connections: 10,
            },
            gui: GuiConfig {
                window_width: 1200,
                window_height: 800,
                theme: "auto".to_string(),
                font_size: 12,
            },
            accounts: vec![],
        }
    }
}

impl Config {
    /// load config from file or create default if it doesn't exist
    pub fn load() -> anyhow::Result<Self> {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("threadbare")
            .join("config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// save config to file
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("threadbare")
            .join("config.toml");

        // create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;
        Ok(())
    }

    /// load config from a specific path
    pub fn load_from(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
