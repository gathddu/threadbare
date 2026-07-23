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
        use gtk4::{Box, Label, Orientation, Paned, CssProvider, SearchEntry};

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

        // compose button
        let compose_btn = gtk4::Button::with_label("Compose");
        compose_btn.add_css_class("suggested-action");
        header_bar.pack_start(&compose_btn);

        // action buttons on the right
        let delete_btn = gtk4::Button::with_label("Delete");
        delete_btn.add_css_class("flat");
        delete_btn.add_css_class("destructive-action");
        header_bar.pack_end(&delete_btn);

        let archive_btn = gtk4::Button::with_label("Archive");
        archive_btn.add_css_class("flat");
        header_bar.pack_end(&archive_btn);
        
        let reply_btn = gtk4::Button::with_label("Reply");
        reply_btn.add_css_class("flat");
        header_bar.pack_end(&reply_btn);

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

        // create content paned (vertical split)
        let content_paned = Paned::new(Orientation::Vertical);
        content_paned.set_hexpand(true);
        content_paned.set_vexpand(true);

        // create e-mail list
        let email_list = Box::new(Orientation::Vertical, 0);
        email_list.add_css_class("email-list");

        let content_label = Label::new(Some("E-mail List"));
        content_label.add_css_class("email-list-header");
        content_label.set_margin_top(10);
        content_label.set_margin_start(10);
        email_list.append(&content_label);

        let search_entry = SearchEntry::new();
        search_entry.set_placeholder_text(Some("Search e-mails.."));
        search_entry.set_margin_start(10);
        search_entry.set_margin_end(10);
        search_entry.set_margin_top(5);
        search_entry.set_margin_bottom(5);
        email_list.append(&search_entry);

        
        let email1 = gtk4::Button::with_label("From: Shaka | Subject: Missing you | 2:30 PM");
        email1.add_css_class("email-item");
        email1.add_css_class("flat");
        email_list.append(&email1);

        let email2 = gtk4::Button::with_label("From: God | Subject: Why are you eeven making this it's 2 AM | 2:15 AM");
        email2.add_css_class("email-item");
        email2.add_css_class("flat");
        email_list.append(&email2);

        let email3 = gtk4::Button::with_label("From: Matl | Subject: we BALL | 12:45 PM");
        email3.add_css_class("email-item");
        email3.add_css_class("flat");
        email_list.append(&email3);

        content_paned.set_start_child(Some(&email_list));
        content_paned.set_position(200);
        content_paned.set_shrink_start_child(false);
        content_paned.set_shrink_end_child(false);

        // create reading pane
        let reading_pane = Box::new(Orientation::Vertical, 0);
        let reading_label = Label::new(Some("Select an e-mail to read"));
        reading_label.set_margin_top(20);
        reading_pane.append(&reading_label);

        content_paned.set_end_child(Some(&reading_pane));

        paned.set_end_child(Some(&content_paned));

        // click handlers for e-mail buttons
        let reading_pane_clone = reading_pane.clone();
        email1.connect_clicked(move |_| {
            while let Some(child) = reading_pane_clone.first_child() {
                reading_pane_clone.remove(&child);
            }
            let from = Label::new(Some("From: Shaka"));
            from.set_margin_top(10);
            from.set_margin_start(10);
            reading_pane_clone.append(&from);

            let subject = Label::new(Some("Subject: Missing you"));
            subject.set_margin_start(10);
            subject.set_margin_top(5);
            reading_pane_clone.append(&subject);

            let body = Label::new(Some("\nlarping larp rn"));
            body.set_margin_start(10);
            body.set_margin_top(10);
            body.set_wrap(true);
            reading_pane_clone.append(&body);
        });

        let reading_pane_clone = reading_pane.clone();
        email2.connect_clicked(move |_| {
            while let Some(child) = reading_pane_clone.first_child() {
                reading_pane_clone.remove(&child);
            }
            let from = Label::new(Some("From: God"));
            from.set_margin_top(10);
            from.set_margin_start(10);
            reading_pane_clone.append(&from);

            let subject = Label::new(Some("Subject: Why are you eeven making this it's 2 AM"));
            subject.set_margin_start(10);
            subject.set_margin_top(5);
            reading_pane_clone.append(&subject);

            let body = Label::new(Some("\nGo to sleep. This can wait until tomorrow."));
            body.set_margin_start(10);
            body.set_margin_top(10);
            body.set_wrap(true);
            reading_pane_clone.append(&body);
        });

        let reading_pane_clone = reading_pane.clone();
        email3.connect_clicked(move |_| {
            while let Some(child) = reading_pane_clone.first_child() {
                reading_pane_clone.remove(&child);
            }
            let from = Label::new(Some("From: Mat"));
            from.set_margin_top(10);
            from.set_margin_start(10);
            reading_pane_clone.append(&from);

            let subject = Label::new(Some("Subject: Re: we BALL"));
            subject.set_margin_start(10);
            subject.set_margin_top(5);
            reading_pane_clone.append(&subject);

            let body = Label::new(Some("\nWE BALL INDEED"));
            body.set_margin_start(10);
            body.set_margin_top(10);
            body.set_wrap(true);
            reading_pane_clone.append(&body);
        });

        // search filter function
        let email1_clone = email1.clone();
        let email2_clone = email2.clone();
        let email3_clone = email3.clone();
        
        search_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_lowercase();
            
            // e-mail data for searching
            let emails = vec![
                ("From: Shaka | Subject: Missing you | 2:30 PM", &email1_clone),
                ("From: God | Subject: Why are you eeven making this it's 2 AM | 2:15 AM", &email2_clone),
                ("From: Mat | Subject: Re: we BALL | 12:45 PM", &email3_clone),
            ];
            
            for (text, btn) in emails {
                if query.is_empty() || text.to_lowercase().contains(&query) {
                    btn.set_visible(true);
                } else {
                    btn.set_visible(false);
                }
            }
        });

        // click handlers for folder buttons
        let email_list_clone = email_list.clone();
        let reading_pane_clone = reading_pane.clone();
        inbox.connect_clicked(move |_| {
            while let Some(child) = email_list_clone.first_child() {
                email_list_clone.remove(&child);
            }
            let header = Label::new(Some("Inbox"));
            header.add_css_class("email-list-header");
            email_list_clone.append(&header);

            let rp = reading_pane_clone.clone();
            let e1 = gtk4::Button::with_label("From: Shaka | Subject: Missing you | 2:30 PM");
            e1.add_css_class("email-item");
            e1.add_css_class("flat");
            e1.connect_clicked(move |_| {
                while let Some(child) = rp.first_child() { rp.remove(&child); }
                let from = Label::new(Some("From: Shaka"));
                from.set_margin_top(10); from.set_margin_start(10);
                rp.append(&from);
                let subject = Label::new(Some("Subject: Missing you"));
                subject.set_margin_start(10); subject.set_margin_top(5);
                rp.append(&subject);
                let body = Label::new(Some("\nlarping larp rn"));
                body.set_margin_start(10); body.set_margin_top(10); body.set_wrap(true);
                rp.append(&body);
            });
            email_list_clone.append(&e1);

            let rp = reading_pane_clone.clone();
            let e2 = gtk4::Button::with_label("From: God | Subject: Why are you even making this it's 2 AM | 2:15 AM");
            e2.add_css_class("email-item");
            e2.add_css_class("flat");
            e2.connect_clicked(move |_| {
                while let Some(child) = rp.first_child() { rp.remove(&child); }
                let from = Label::new(Some("From: God"));
                from.set_margin_top(10); from.set_margin_start(10);
                rp.append(&from);
                let subject = Label::new(Some("Subject: Why are you even making this it's 2 AM"));
                subject.set_margin_start(10); subject.set_margin_top(5);
                rp.append(&subject);
                let body = Label::new(Some("\nGo to sleep. This can wait until tomorrow."));
                body.set_margin_start(10); body.set_margin_top(10); body.set_wrap(true);
                rp.append(&body);
            });
            email_list_clone.append(&e2);

            let rp = reading_pane_clone.clone();
            let e3 = gtk4::Button::with_label("From: Mat | Subject: we BALL | 12:45 PM");
            e3.add_css_class("email-item");
            e3.add_css_class("flat");
            e3.connect_clicked(move |_| {
                while let Some(child) = rp.first_child() { rp.remove(&child); }
                let from = Label::new(Some("From: Mat"));
                from.set_margin_top(10); from.set_margin_start(10);
                rp.append(&from);
                let subject = Label::new(Some("Subject: Re: we BALL"));
                subject.set_margin_start(10); subject.set_margin_top(5);
                rp.append(&subject);
                let body = Label::new(Some("\nWE BALL INDEED"));
                body.set_margin_start(10); body.set_margin_top(10); body.set_wrap(true);
                rp.append(&body);
            });
            email_list_clone.append(&e3);
        });

        let email_list_clone = email_list.clone();
        let reading_pane_clone = reading_pane.clone();
        sent.connect_clicked(move |_| {
            while let Some(child) = email_list_clone.first_child() {
                email_list_clone.remove(&child);
            }
            let header = Label::new(Some("Sent"));
            header.add_css_class("email-list-header");
            email_list_clone.append(&header);

            let rp = reading_pane_clone.clone();
            let e1 = gtk4::Button::with_label("To: Bella | Subject: yo check this out | 1:00 PM");
            e1.add_css_class("email-item");
            e1.add_css_class("flat");
            e1.connect_clicked(move |_| {
                while let Some(child) = rp.first_child() { rp.remove(&child); }
                let from = Label::new(Some("To: Bella"));
                from.set_margin_top(10); from.set_margin_start(10);
                rp.append(&from);
                let subject = Label::new(Some("Subject: yo check this out"));
                subject.set_margin_start(10); subject.set_margin_top(5);
                rp.append(&subject);
                let body = Label::new(Some("\ncheck this out fr fr"));
                body.set_margin_start(10); body.set_margin_top(10); body.set_wrap(true);
                rp.append(&body);
            });
            email_list_clone.append(&e1);

            let rp = reading_pane_clone.clone();
            let e2 = gtk4::Button::with_label("To: Shaka | Subject: Re: Missing you | 3:00 PM");
            e2.add_css_class("email-item");
            e2.add_css_class("flat");
            e2.connect_clicked(move |_| {
                while let Some(child) = rp.first_child() { rp.remove(&child); }
                let from = Label::new(Some("To: Shaka"));
                from.set_margin_top(10); from.set_margin_start(10);
                rp.append(&from);
                let subject = Label::new(Some("Subject: Re: Missing you"));
                subject.set_margin_start(10); subject.set_margin_top(5);
                rp.append(&subject);
                let body = Label::new(Some("\nmissing u too bro"));
                body.set_margin_start(10); body.set_margin_top(10); body.set_wrap(true);
                rp.append(&body);
            });
            email_list_clone.append(&e2);
        });

        let email_list_clone = email_list.clone();
        let reading_pane_clone = reading_pane.clone();
        drafts.connect_clicked(move |_| {
            while let Some(child) = email_list_clone.first_child() {
                email_list_clone.remove(&child);
            }
            let header = Label::new(Some("Drafts"));
            header.add_css_class("email-list-header");
            email_list_clone.append(&header);

            let rp = reading_pane_clone.clone();
            let e1 = gtk4::Button::with_label("To: Mom | Subject: iupi | (unsent)");
            e1.add_css_class("email-item");
            e1.add_css_class("flat");
            e1.connect_clicked(move |_| {
                while let Some(child) = rp.first_child() { rp.remove(&child); }
                let from = Label::new(Some("To: Mom"));
                from.set_margin_top(10); from.set_margin_start(10);
                rp.append(&from);
                let subject = Label::new(Some("Subject: iupi"));
                subject.set_margin_start(10); subject.set_margin_top(5);
                rp.append(&subject);
                let body = Label::new(Some("\n[draft content]"));
                body.set_margin_start(10); body.set_margin_top(10); body.set_wrap(true);
                rp.append(&body);
            });
            email_list_clone.append(&e1);
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

