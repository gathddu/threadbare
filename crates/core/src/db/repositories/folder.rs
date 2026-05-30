//! folder repository for database operations

use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::folder::{Folder, FolderType};
use crate::Result;

/// repository for folder operations
pub struct FolderRepository;

impl FolderRepository {
    /// create a new folder
    pub async fn create(pool: &SqlitePool, folder: &Folder) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO folders (
                id, account_id, name, imap_path, email_count,
                unread_count, folder_type, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(folder.id.to_string())
        .bind(folder.account_id.to_string())
        .bind(&folder.name)
        .bind(&folder.imap_path)
        .bind(folder.email_count)
        .bind(folder.unread_count)
        .bind(folder.folder_type.as_str())
        .bind(folder.created_at.to_rfc3339())
        .bind(folder.updated_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(())
    }

    /// get folder by ID
    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Folder>> {
        let row = sqlx::query_as::<_, FolderRow>(
            "SELECT * FROM folders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_folder()))
    }

    /// get all folders for an account
    pub async fn get_by_account(pool: &SqlitePool, account_id: Uuid) -> Result<Vec<Folder>> {
        let rows = sqlx::query_as::<_, FolderRow>(
            "SELECT * FROM folders WHERE account_id = ? ORDER BY name ASC"
        )
        .bind(account_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_folder()).collect())
    }

    /// update folder
    pub async fn update(pool: &SqlitePool, folder: &Folder) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE folders SET
                name = ?, email_count = ?, unread_count = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&folder.name)
        .bind(folder.email_count)
        .bind(folder.unread_count)
        .bind(folder.updated_at.to_rfc3339())
        .bind(folder.id.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    /// delete folder
    pub async fn delete(pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM folders WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;

        Ok(())
    }
}

/// internal row type for database queries
#[derive(sqlx::FromRow)]
struct FolderRow {
    id: String,
    account_id: String,
    name: String,
    imap_path: String,
    email_count: i32,
    unread_count: i32,
    folder_type: String,
    created_at: String,
    updated_at: String,
}

impl FolderRow {
    fn into_folder(self) -> Folder {
        Folder {
            id: Uuid::parse_str(&self.id).unwrap_or_else(|_| Uuid::new_v4()),
            account_id: Uuid::parse_str(&self.account_id).unwrap_or_else(|_| Uuid::new_v4()),
            name: self.name,
            imap_path: self.imap_path,
            email_count: self.email_count as u32,
            unread_count: self.unread_count as u32,
            folder_type: FolderType::from_str(&self.folder_type),
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).ok().map(|dt| dt.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}

