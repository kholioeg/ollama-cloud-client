use crate::error::Result;
use crate::api::Message;
use log::info;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use uuid::Uuid;
use chrono::Utc;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self> {
        // Ensure directory exists
        if let Some(parent) = std::path::Path::new(db_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Check if database file exists
        let db_path_obj = std::path::Path::new(db_path);
        let is_new_db = !db_path_obj.exists();

        let database_url = format!("sqlite://{}", db_path);
        if is_new_db {
            info!("Creating new database at: {}", database_url);
        } else {
            info!("Using existing database at: {}", database_url);
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        let db = Database { pool };
        db.init_schema().await?;

        Ok(db)
    }

    async fn init_schema(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                model TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        info!("Database schema initialized");
        Ok(())
    }

    pub async fn create_conversation(&self, title: &str, model: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO conversations (id, title, model, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(title)
        .bind(model)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn save_message(&self, conversation_id: &str, message: &Message) -> Result<()> {
        sqlx::query(
            "INSERT INTO messages (id, conversation_id, role, content, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&message.id)
        .bind(conversation_id)
        .bind(format!("{:?}", message.role))
        .bind(&message.content)
        .bind(message.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_conversation_messages(&self, conversation_id: &str) -> Result<Vec<Message>> {
        // TODO: Implement message retrieval
        let _messages = sqlx::query("SELECT * FROM messages WHERE conversation_id = ?")
            .bind(conversation_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(vec![])
    }

    pub async fn list_conversations(&self) -> Result<Vec<(String, String)>> {
        let rows = sqlx::query("SELECT id, title FROM conversations ORDER BY updated_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let conversations = rows
            .into_iter()
            .map(|row| {
                let id: String = row.get(0);
                let title: String = row.get(1);
                (id, title)
            })
            .collect();

        Ok(conversations)
    }
}
