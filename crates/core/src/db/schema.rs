//! database schema definitions

use sqlx::sqlite::SqlitePool;
use crate::Result;

/// initialize the database and run migrations
pub async fn init(pool: &SqlitePool) -> Result<()> {
    // create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            imap_host TEXT NOT NULL,
            imap_port INTEGER NOT NULL,
            imap_tls BOOLEAN NOT NULL DEFAULT 1,
            smtp_host TEXT NOT NULL,
            smtp_port INTEGER NOT NULL,
            smtp_tls BOOLEAN NOT NULL DEFAULT 1,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            is_enabled BOOLEAN NOT NULL DEFAULT 1,
            last_synced_at TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            account_id TEXT NOT NULL,
            name TEXT NOT NULL,
            imap_path TEXT NOT NULL,
            email_count INTEGER NOT NULL DEFAULT 0,
            unread_count INTEGER NOT NULL DEFAULT 0,
            folder_type TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
            UNIQUE(account_id, imap_path)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS emails (
            id TEXT PRIMARY KEY,
            account_id TEXT NOT NULL,
            folder_id TEXT NOT NULL,
            message_id TEXT NOT NULL,
            subject TEXT NOT NULL,
            from_addr TEXT NOT NULL,
            to_addr TEXT NOT NULL,
            cc_addr TEXT,
            bcc_addr TEXT,
            reply_to TEXT,
            body_text TEXT NOT NULL,
            body_html TEXT,
            received_at TEXT NOT NULL,
            sent_at TEXT NOT NULL,
            is_read BOOLEAN NOT NULL DEFAULT 0,
            is_starred BOOLEAN NOT NULL DEFAULT 0,
            is_spam BOOLEAN NOT NULL DEFAULT 0,
            thread_id TEXT,
            raw_data BLOB,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
            FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE,
            UNIQUE(account_id, message_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // create indexes for common queries
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_emails_account_id ON emails(account_id)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_emails_folder_id ON emails(folder_id)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_emails_is_read ON emails(is_read)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_emails_thread_id ON emails(thread_id)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_folders_account_id ON folders(account_id)
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

