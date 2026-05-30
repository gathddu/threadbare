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
        use gtk4::{Box, Label, Orientation, Paned};

        // create main window
        let window = libadwaita::ApplicationWindow::builder()
            .application(app)
            .title("Threadbare E-mail")
            .default_width(1200)
            .default_height(800)
            .build();

        // create header bar
        let header_bar = libadwaita::HeaderBar::new();

        // create main box (vertical)
        let main_box = Box::new(Orientation::Vertical, 0);
        main_box.set_hexpand(true);
        main_box.set_vexpand(true);

        // add header bar
        main_box.append(&header_bar);

        // create paned widget (horizontal split)
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_hexpand(true);
        paned.set_vexpand(true);

        // create sidebar
        let sidebar = Box::new(Orientation::Vertical, 0);
        let sidebar_label = Label::new(Some("Accounts & Folders"));
        sidebar_label.set_margin_top(10);
        sidebar_label.set_margin_start(10);
        sidebar.append(&sidebar_label);
        paned.set_start_child(Some(&sidebar));
        paned.set_position(250);

        let account1 = Label::new(Some("jessforsterdev@gmail.com"));
        account1.set_margin_start(15);
        account1.set_margin_top(5);
        sidebar.append(&account1);

        let inbox = Label::new(Some("Inbox (5)"));
        inbox.set_margin_start(25);
        inbox.set_margin_top(3);
        sidebar.append(&inbox);

        let sent = Label::new(Some("Sent"));
        sent.set_margin_start(25);
        sent.set_margin_top(3);
        sidebar.append(&sent);

        let drafts = Label::new(Some("Drafts"));
        drafts.set_margin_start(25);
        drafts.set_margin_top(3);
        sidebar.append(&drafts);


        // create content area
        let content_area = Box::new(Orientation::Vertical, 0);
        let content_label = Label::new(Some("E-mail List"));
        content_label.set_margin_top(10);
        content_label.set_margin_start(10);
        content_area.append(&content_label);
        
        let email1 = Label::new(Some("From: Shaka | Subject: Missing you | 2:30 PM"));
        email1.set_margin_start(15);
        email1.set_margin_top(10);
        content_area.append(&email1);

        let email2 = Label::new(Some("From: God | Subject: Why are you eeven making this it's 2 AM | 2:15 AM"));
        email2.set_margin_start(15);
        email2.set_margin_top(5);
        content_area.append(&email2);

        let email3 = Label::new(Some("From: Matl | Subject: Re: we BALL | 12:45 PM"));
        email3.set_margin_start(15);
        email3.set_margin_top(5);
        content_area.append(&email3);

        paned.set_end_child(Some(&content_area));

        // add paned to main box
        main_box.append(&paned);

        window.set_content(Some(&main_box));
        window.present();
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

