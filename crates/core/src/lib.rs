//! threadbare-core
//! 
//! core library for threadbare e-mail client.
//! shared types, e-mail parsing, database models and business logic.

pub mod error;
pub mod models;
pub mod config;

pub use error::{Error, Result};

/// library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

