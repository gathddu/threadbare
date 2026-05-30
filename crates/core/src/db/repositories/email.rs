//! e-mail repository for database operations

use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::Email;
use crate::Result;

/// repository for e-mail operations
pub struct EmailRepository;

impl EmailRepository {
    /// create a new e-mail
    pub async fn create(pool: &SqlitePool, email: &Email) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO emails (
                id, account_id, folder_id, message_id, subject,
                from_addr, to_addr, cc_addr, bcc_addr, reply_to,
                body_text, body_html, received_at, sent_at,
                is_read, is_starred, is_spam, thread_id, raw_data,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(email.id.to_string())
        .bind(email.account_id.to_string())
        .bind(email.folder_id.to_string())
        .bind(&email.message_id)
        .bind(&email.subject)
        .bind(&email.from)
        .bind(&email.to)
        .bind(&email.cc)
        .bind(&email.bcc)
        .bind(&email.reply_to)
        .bind(&email.body_text)
        .bind(&email.body_html)
        .bind(email.received_at.to_rfc3339())
        .bind(email.sent_at.to_rfc3339())
        .bind(email.is_read)
        .bind(email.is_starred)
        .bind(email.is_spam)
        .bind(&email.thread_id)
        .bind(&email.raw_data)
        .bind(email.created_at.to_rfc3339())
        .bind(email.updated_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(())
    }

    /// get e-mail by ID
    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Email>> {
        let row = sqlx::query_as::<_, EmailRow>(
            "SELECT * FROM emails WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_email()))
    }

    /// get all e-mails in a folder
    pub async fn get_by_folder(pool: &SqlitePool, folder_id: Uuid) -> Result<Vec<Email>> {
        let rows = sqlx::query_as::<_, EmailRow>(
            "SELECT * FROM emails WHERE folder_id = ? ORDER BY received_at DESC"
        )
        .bind(folder_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_email()).collect())
    }

    /// get unread e-mails in a folder
    pub async fn get_unread_by_folder(pool: &SqlitePool, folder_id: Uuid) -> Result<Vec<Email>> {
        let rows = sqlx::query_as::<_, EmailRow>(
            "SELECT * FROM emails WHERE folder_id = ? AND is_read = 0 ORDER BY received_at DESC"
        )
        .bind(folder_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_email()).collect())
    }

    /// update e-mail
    pub async fn update(pool: &SqlitePool, email: &Email) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE emails SET
                subject = ?, from_addr = ?, to_addr = ?, cc_addr = ?,
                bcc_addr = ?, reply_to = ?, body_text = ?, body_html = ?,
                is_read = ?, is_starred = ?, is_spam = ?, thread_id = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&email.subject)
        .bind(&email.from)
        .bind(&email.to)
        .bind(&email.cc)
        .bind(&email.bcc)
        .bind(&email.reply_to)
        .bind(&email.body_text)
        .bind(&email.body_html)
        .bind(email.is_read)
        .bind(email.is_starred)
        .bind(email.is_spam)
        .bind(&email.thread_id)
        .bind(email.updated_at.to_rfc3339())
        .bind(email.id.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    /// delete e-mail
    pub async fn delete(pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM emails WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// internal row type for database queries
#[derive(sqlx::FromRow)]
struct EmailRow {
    id: String,
    account_id: String,
    folder_id: String,
    message_id: String,
    subject: String,
    from_addr: String,
    to_addr: String,
    cc_addr: Option<String>,
    bcc_addr: Option<String>,
    reply_to: Option<String>,
    body_text: String,
    body_html: Option<String>,
    received_at: String,
    sent_at: String,
    is_read: bool,
    is_starred: bool,
    is_spam: bool,
    thread_id: Option<String>,
    raw_data: Option<Vec<u8>>,
    created_at: String,
    updated_at: String,
}

impl EmailRow {
    fn into_email(self) -> Email {
        Email {
            id: Uuid::parse_str(&self.id).unwrap_or_else(|_| Uuid::new_v4()),
            account_id: Uuid::parse_str(&self.account_id).unwrap_or_else(|_| Uuid::new_v4()),
            folder_id: Uuid::parse_str(&self.folder_id).unwrap_or_else(|_| Uuid::new_v4()),
            message_id: self.message_id,
            subject: self.subject,
            from: self.from_addr,
            to: self.to_addr,
            cc: self.cc_addr,
            bcc: self.bcc_addr,
            reply_to: self.reply_to,
            body_text: self.body_text,
            body_html: self.body_html,
            received_at: chrono::DateTime::parse_from_rfc3339(&self.received_at).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            sent_at: chrono::DateTime::parse_from_rfc3339(&self.sent_at).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            is_read: self.is_read,
            is_starred: self.is_starred,
            is_spam: self.is_spam,
            thread_id: self.thread_id,
            raw_data: self.raw_data,
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}

