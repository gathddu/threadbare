//! account repository for database operations

use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::Account;
use crate::Result;

/// repository for account operations
pub struct AccountRepository;

impl AccountRepository {
    /// create a new account
    pub async fn create(pool: &SqlitePool, account: &Account) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO accounts (
                id, name, email, imap_host, imap_port, imap_tls,
                smtp_host, smtp_port, smtp_tls, username, password,
                is_enabled, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(account.id.to_string())
        .bind(&account.name)
        .bind(&account.email)
        .bind(&account.imap_host)
        .bind(account.imap_port)
        .bind(account.imap_tls)
        .bind(&account.smtp_host)
        .bind(account.smtp_port)
        .bind(account.smtp_tls)
        .bind(&account.username)
        .bind(&account.password)
        .bind(account.is_enabled)
        .bind(account.created_at.to_rfc3339())
        .bind(account.updated_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(())
    }

    /// get account by ID
    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Account>> {
        let row = sqlx::query_as::<_, AccountRow>(
            "SELECT * FROM accounts WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_account()))
    }

    /// get all accounts
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Account>> {
        let rows = sqlx::query_as::<_, AccountRow>(
            "SELECT * FROM accounts ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_account()).collect())
    }

    /// get enabled accounts
    pub async fn get_enabled(pool: &SqlitePool) -> Result<Vec<Account>> {
        let rows = sqlx::query_as::<_, AccountRow>(
            "SELECT * FROM accounts WHERE is_enabled = 1 ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_account()).collect())
    }

    /// update account
    pub async fn update(pool: &SqlitePool, account: &Account) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE accounts SET
                name = ?, email = ?, imap_host = ?, imap_port = ?,
                imap_tls = ?, smtp_host = ?, smtp_port = ?, smtp_tls = ?,
                username = ?, password = ?, is_enabled = ?,
                last_synced_at = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&account.name)
        .bind(&account.email)
        .bind(&account.imap_host)
        .bind(account.imap_port)
        .bind(account.imap_tls)
        .bind(&account.smtp_host)
        .bind(account.smtp_port)
        .bind(account.smtp_tls)
        .bind(&account.username)
        .bind(&account.password)
        .bind(account.is_enabled)
        .bind(account.last_synced_at.map(|t| t.to_rfc3339()))
        .bind(account.updated_at.to_rfc3339())
        .bind(account.id.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    /// delete account
    pub async fn delete(pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM accounts WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// internal row type for database queries
#[derive(sqlx::FromRow)]
struct AccountRow {
    id: String,
    name: String,
    email: String,
    imap_host: String,
    imap_port: i32,
    imap_tls: bool,
    smtp_host: String,
    smtp_port: i32,
    smtp_tls: bool,
    username: String,
    password: String,
    is_enabled: bool,
    last_synced_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl AccountRow {
    fn into_account(self) -> Account {
        Account {
            id: Uuid::parse_str(&self.id).unwrap_or_else(|_| Uuid::new_v4()),
            name: self.name,
            email: self.email,
            imap_host: self.imap_host,
            imap_port: self.imap_port as u16,
            imap_tls: self.imap_tls,
            smtp_host: self.smtp_host,
            smtp_port: self.smtp_port as u16,
            smtp_tls: self.smtp_tls,
            username: self.username,
            password: self.password,
            is_enabled: self.is_enabled,
            last_synced_at: self.last_synced_at.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Utc))),
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}

