//! e-mail message model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// represents an e-mail message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// unique identifier
    pub id: Uuid,
    
    /// account ID
    pub account_id: Uuid,
    
    /// folder ID
    pub folder_id: Uuid,
    
    /// message ID (from e-mail headers)
    pub message_id: String,
    
    /// subject line
    pub subject: String,
    
    /// from address
    pub from: String,
    
    /// to addresses (comma-separated)
    pub to: String,
    
    /// CC addresses (comma-separated)
    pub cc: Option<String>,
    
    /// BCC addresses (comma-separated)
    pub bcc: Option<String>,
    
    /// reply-to address
    pub reply_to: Option<String>,
    
    /// e-mail body (text)
    pub body_text: String,
    
    /// e-mail body (HTML)
    pub body_html: Option<String>,
    
    /// when the e-mail was received
    pub received_at: DateTime<Utc>,
    
    /// when the e-mail was sent
    pub sent_at: DateTime<Utc>,
    
    /// e-mail read
    pub is_read: bool,
    
    /// e-mail starred/flagged
    pub is_starred: bool,
    
    /// e-mail spam
    pub is_spam: bool,
    
    /// Thread ID (for grouping conversations)
    pub thread_id: Option<String>,
    
    /// raw e-mail data (for re-parsing)
    pub raw_data: Option<Vec<u8>>,
    
    /// when the record was created
    pub created_at: DateTime<Utc>,
    
    /// when the record was last updated
    pub updated_at: DateTime<Utc>,
}

impl Email {
    /// create a new e-mail
    pub fn new(
        account_id: Uuid,
        folder_id: Uuid,
        message_id: String,
        subject: String,
        from: String,
        to: String,
        body_text: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            account_id,
            folder_id,
            message_id,
            subject,
            from,
            to,
            cc: None,
            bcc: None,
            reply_to: None,
            body_text,
            body_html: None,
            received_at: now,
            sent_at: now,
            is_read: false,
            is_starred: false,
            is_spam: false,
            thread_id: None,
            raw_data: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// mark e-mail as read
    pub fn mark_read(&mut self) {
        self.is_read = true;
        self.updated_at = Utc::now();
    }

    /// mark e-mail as unread
    pub fn mark_unread(&mut self) {
        self.is_read = false;
        self.updated_at = Utc::now();
    }

    /// toggle star status
    pub fn toggle_star(&mut self) {
        self.is_starred = !self.is_starred;
        self.updated_at = Utc::now();
    }
}

