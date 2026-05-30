//! main application structure

use gtk4::prelude::*;
use gtk4::Application as GtkApplication;
use libadwaita::prelude::*;

/// main threadbare application
pub struct Application {
    gtk_app: GtkApplication,
}

impl Application {
    /// create a new application
    pub fn new() -> Self {
        let gtk_app = GtkApplication::builder()
            .application_id("com.threadbare.gui")
            .build();

        gtk_app.connect_activate(|app| {
            Self::on_activate(app);
        });

        Self { gtk_app }
    }

    /// run the application
    pub fn run(&self) {
        let args: Vec<String> = std::env::args().collect();
        self.gtk_app.run_with_args(&args);
    }

    /// handle application activation
    fn on_activate(app: &GtkApplication) {
        // Create main window
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Threadbare E-mail")
            .default_width(1200)
            .default_height(800)
            .build();

        // create header bar
        let header_bar = libadwaita::HeaderBar::new();

        // create main box
        let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        main_box.append(&header_bar);

        // create content area (placeholder)
        let content = gtk4::Label::new(Some("E-mail client loading..."));
        content.set_margin_top(20);
        content.set_margin_bottom(20);
        content.set_margin_start(20);
        content.set_margin_end(20);
        main_box.append(&content);

        window.set_content(Some(&main_box));
        window.present();
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

