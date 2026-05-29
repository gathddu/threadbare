//! e-mail folder model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// e-mail folder/mailbox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// unique identifier
    pub id: Uuid,
    
    /// Account ID
    pub account_id: Uuid,
    
    /// folder name
    pub name: String,
    
    /// IMAP folder path
    pub imap_path: String,
    
    /// number of e-mails in folder
    pub email_count: u32,
    
    /// number of unread emails
    pub unread_count: u32,
    
    /// special folder? (inbox, sent, drafts, etc.)
    pub folder_type: FolderType,
    
    /// when the record was created
    pub created_at: DateTime<Utc>,
    
    /// when the record was last updated
    pub updated_at: DateTime<Utc>,
}

/// types of special folders
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FolderType {
    Inbox,
    Sent,
    Drafts,
    Trash,
    Spam,
    Archive,
    Custom,
}

impl Folder {
    /// create a new folder
    pub fn new(
        account_id: Uuid,
        name: String,
        imap_path: String,
        folder_type: FolderType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            account_id,
            name,
            imap_path,
            email_count: 0,
            unread_count: 0,
            folder_type,
            created_at: now,
            updated_at: now,
        }
    }
}

