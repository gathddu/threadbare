//! e-mail list widget

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, ScrolledWindow};

/// e-mail list widget
pub struct EmailListWidget;

impl EmailListWidget {
    /// create a new e-mail list widget
    pub fn new() -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 0);
        
        // create scrolled window
        let scrolled = ScrolledWindow::new();
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);

        // placeholder content
        let label = Label::new(Some("No E-mails loaded yet"));
        label.set_margin_top(20);
        label.set_margin_bottom(20);
        label.set_margin_start(20);
        label.set_margin_end(20);

        scrolled.set_child(Some(&label));
        container.append(&scrolled);

        container
    }
}

impl Default for EmailListWidget {
    fn default() -> Self {
        Self
    }
}

