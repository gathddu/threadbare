//! threadbare-daemon
//! 
//! daemon for threadbare e-mail client.
//! handles IMAP sync, database management and IPC communication

pub mod server;
pub mod sync;
pub mod state;

pub use server::DaemonServer;
pub use state::DaemonState;

/// version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

