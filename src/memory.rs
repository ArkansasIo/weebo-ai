use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use uuid::Uuid;
use crate::models::Message;

#[derive(Clone)]
pub struct MemoryStore { pool: SqlitePool }

impl MemoryStore {
    pub async fn connect(url: &str) -> Result<Self> {
        if url.starts_with("sqlite://") {
            let file = url.trim_start_matches("sqlite://");
            if file != ":memory:" {
                if let Some(parent) = std::path::Path::new(file).parent() {
                    if !parent.as_os_str().is_empty() { tokio::fs::create_dir_all(parent).await?; }
                }
            }
        }
        let pool = SqlitePoolOptions::new().max_connections(5).connect(url).await?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::query(include_str!("../migrations/001_initial.sql")).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn store_message(&self, conversation_id: Uuid, message: &Message) -> Result<()> {
        let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM conversations WHERE id=?1")
            .bind(conversation_id.to_string()).fetch_optional(&self.pool).await?;
        let now = chrono::Utc::now();
        if exists.is_none() {
            sqlx::query("INSERT INTO conversations (id,title,created_at,updated_at) VALUES (?1,?2,?3,?4)")
                .bind(conversation_id.to_string())
                .bind(message.content.chars().take(48).collect::<String>())
                .bind(now).bind(now).execute(&self.pool).await?;
        }
        sqlx::query("INSERT INTO messages (id,conversation_id,role,content,created_at) VALUES (?1,?2,?3,?4,?5)")
            .bind(Uuid::new_v4().to_string()).bind(conversation_id.to_string())
            .bind(&message.role).bind(&message.content).bind(now).execute(&self.pool).await?;
        sqlx::query("UPDATE conversations SET updated_at=?1 WHERE id=?2")
            .bind(now).bind(conversation_id.to_string()).execute(&self.pool).await?;
        Ok(())
    }
}
