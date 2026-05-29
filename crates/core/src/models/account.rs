//! e-mail account model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// e-mail account configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// unique identifier
    pub id: Uuid,
    
    /// display name
    pub name: String,
    
    /// e-mail address
    pub email: String,
    
    /// IMAP server hostname
    pub imap_host: String,
    
    /// IMAP server port
    pub imap_port: u16,
    
    /// use TLS for IMAP?
    pub imap_tls: bool,
    
    /// SMTP server hostname
    pub smtp_host: String,
    
    /// SMTP server port
    pub smtp_port: u16,
    
    /// use TLS for SMTP?
    pub smtp_tls: bool,
    
    /// username for authentication
    pub username: String,
    
    /// password (encrypted)
    pub password: String,
    
    /// is this account enabled?
    pub is_enabled: bool,
    
    /// last sync time
    pub last_synced_at: Option<DateTime<Utc>>,
    
    /// when the record was created
    pub created_at: DateTime<Utc>,
    
    /// when the record was last updated
    pub updated_at: DateTime<Utc>,
}

impl Account {
    /// create a new account
    pub fn new(
        name: String,
        email: String,
        imap_host: String,
        imap_port: u16,
        smtp_host: String,
        smtp_port: u16,
        username: String,
        password: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            imap_host,
            imap_port,
            imap_tls: true,
            smtp_host,
            smtp_port,
            smtp_tls: true,
            username,
            password,
            is_enabled: true,
            last_synced_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// mark account as synced
    pub fn mark_synced(&mut self) {
        self.last_synced_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

