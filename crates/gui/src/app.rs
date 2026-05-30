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
        use gtk4::{Box, Label, Orientation, Paned, CssProvider};

        // load CSS
        let css_provider = CssProvider::new();
        css_provider.load_from_string(include_str!("style.css"));
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

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
        sidebar.add_css_class("sidebar");

        let sidebar_label = Label::new(Some("Accounts & Folders"));
        sidebar_label.add_css_class("sidebar-header");
        sidebar_label.set_margin_top(10);
        sidebar_label.set_margin_start(10);
        sidebar.append(&sidebar_label);

        let account1 = Label::new(Some("jessforsterdev@gmail.com"));
        account1.add_css_class("account-item");
        account1.set_margin_start(15);
        account1.set_margin_top(5);
        sidebar.append(&account1);

        let inbox = gtk4::Button::with_label("Inbox (5)");
        inbox.add_css_class("folder-item");
        inbox.add_css_class("flat");
        inbox.set_margin_start(25);
        inbox.set_margin_top(3);
        sidebar.append(&inbox);

        let sent = gtk4::Button::with_label("Sent");
        sent.add_css_class("folder-item");
        sent.add_css_class("flat");
        sent.set_margin_start(25);
        sent.set_margin_top(3);
        sidebar.append(&sent);

        let drafts = gtk4::Button::with_label("Drafts");
        drafts.add_css_class("folder-item");
        drafts.add_css_class("flat");
        drafts.set_margin_start(25);
        drafts.set_margin_top(3);
        sidebar.append(&drafts);

        paned.set_start_child(Some(&sidebar));
        paned.set_position(250);

        // create content area
        let content_area = Box::new(Orientation::Vertical, 0);
        content_area.add_css_class("email-list");

        let content_label = Label::new(Some("E-mail List"));
        content_label.add_css_class("email-list-header");
        content_label.set_margin_top(10);
        content_label.set_margin_start(10);
        content_area.append(&content_label);
        
        let email1 = Label::new(Some("From: Shaka | Subject: Missing you | 2:30 PM"));
        email1.add_css_class("email-item");
        email1.set_margin_start(15);
        email1.set_margin_top(10);
        content_area.append(&email1);

        let email2 = Label::new(Some("From: God | Subject: Why are you eeven making this it's 2 AM | 2:15 AM"));
        email2.add_css_class("email-item");
        email2.set_margin_start(15);
        email2.set_margin_top(5);
        content_area.append(&email2);

        let email3 = Label::new(Some("From: Matl | Subject: Re: we BALL | 12:45 PM"));
        email3.add_css_class("email-item");
        email3.set_margin_start(15);
        email3.set_margin_top(5);
        content_area.append(&email3);

        paned.set_end_child(Some(&content_area));

        // click handlers for folder buttons
        let content_area_clone = content_area.clone();
        inbox.connect_clicked(move |_| {
        // clear existing children
        while let Some(child) = content_area_clone.first_child() {
            content_area_clone.remove(&child);
        }
        let header = Label::new(Some("Inbox"));
        header.add_css_class("email-list-header");
        content_area_clone.append(&header);

        let e1 = Label::new(Some("From: Shaka | Subject: Missing you | 2:30 PM"));
        e1.add_css_class("email-item");
        content_area_clone.append(&e1);

        let e2 = Label::new(Some("From: God | Subject: Why are you even making this it's 2 AM | 2:15 AM"));
        e2.add_css_class("email-item");
        content_area_clone.append(&e2);

        let e3 = Label::new(Some("From: Mat | Subject: we BALL | 12:45 PM"));
        e3.add_css_class("email-item");
        content_area_clone.append(&e3);
        });

        let content_area_clone = content_area.clone();
        sent.connect_clicked(move |_| {
            while let Some(child) = content_area_clone.first_child() {
                content_area_clone.remove(&child);
            }
            let header = Label::new(Some("Sent"));
            header.add_css_class("email-list-header");
            content_area_clone.append(&header);

            let e1 = Label::new(Some("To: Bella | Subject: yo check this out | 1:00 PM"));
            e1.add_css_class("email-item");
            content_area_clone.append(&e1);

            let e2 = Label::new(Some("To: Shaka | Subject: Re: Missing you | 3:00 PM"));
            e2.add_css_class("email-item");
            content_area_clone.append(&e2);
        });

        let content_area_clone = content_area.clone();
        drafts.connect_clicked(move |_| {
            while let Some(child) = content_area_clone.first_child() {
                content_area_clone.remove(&child);
            }
            let header = Label::new(Some("Drafts"));
            header.add_css_class("email-list-header");
            content_area_clone.append(&header);

            let e1 = Label::new(Some("To: Mom | Subject: iupi | (unsent)"));
            e1.add_css_class("email-item");
            content_area_clone.append(&e1);
        });

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

