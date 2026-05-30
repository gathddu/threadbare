//! sidebar widget for accounts and folders

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, ScrolledWindow};

/// sidebar widget
pub struct SidebarWidget;

impl SidebarWidget {
    /// create a new sidebar widget
    pub fn new() -> GtkBox {
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_size_request(250, -1);

        // title
        let title = Label::new(Some("Accounts"));
        title.set_margin_top(10);
        title.set_margin_bottom(10);
        title.set_margin_start(10);
        title.set_margin_end(10);
        container.append(&title);

        // scrolled area for accounts/folders
        let scrolled = ScrolledWindow::new();
        scrolled.set_hexpand(true);
        scrolled.set_vexpand(true);

        let placeholder = Label::new(Some("No accounts configured"));
        placeholder.set_margin_top(10);
        placeholder.set_margin_bottom(10);
        placeholder.set_margin_start(10);
        placeholder.set_margin_end(10);

        scrolled.set_child(Some(&placeholder));
        container.append(&scrolled);

        container
    }
}

impl Default for SidebarWidget {
    fn default() -> Self {
        Self
    }
}

