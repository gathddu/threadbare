//! threadbare-gui
//! 
//! graphical user interface for threadbare e-mail client.
//! built with GTK4 and libadwaita.

pub mod app;
pub mod windows;
pub mod widgets;

pub use app::Application;

/// version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

