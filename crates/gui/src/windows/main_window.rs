//! main application window

use gtk4::prelude::*;
use gtk4::Box as GtkBox;
use gtk4::Orientation;
use libadwaita::prelude::*;
use libadwaita::{ApplicationWindow, HeaderBar};

/// main application window
pub struct MainWindow;

impl MainWindow {
    /// create the main window
    pub fn new(app: &libadwaita::Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Threadbare E-mail")
            .default_width(1200)
            .default_height(800)
            .build();

        // create header bar
        let header_bar = HeaderBar::new();

        // create main layout
        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.append(&header_bar);

        // create sidebar + content pane
        let content_box = GtkBox::new(Orientation::Horizontal, 0);
        
        // sidebar (accounts/folders)
        let sidebar = GtkBox::new(Orientation::Vertical, 0);
        sidebar.set_size_request(250, -1);
        let sidebar_label = gtk4::Label::new(Some("Accounts & Folders"));
        sidebar.append(&sidebar_label);
        
        // main content
        let content_area = GtkBox::new(Orientation::Vertical, 0);
        let content_label = gtk4::Label::new(Some("E-mail List"));
        content_area.append(&content_label);

        content_box.append(&sidebar);
        content_box.append(&content_area);
        content_box.set_hexpand(true);
        content_box.set_vexpand(true);

        main_box.append(&content_box);

        window.set_content(Some(&main_box));
        window
    }
}

