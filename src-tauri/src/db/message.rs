use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use crate::db::get_pool;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Message {
    pub id: String,
    pub chat_id: String,
    pub role: String,
    pub content: String,
    pub attachments: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub async fn create(
        id: &str,
        chat_id: &str,
        role: &str,
        content: &str,
        attachments: Option<&str>,
    ) -> Result<Self, sqlx::Error> {
        let pool = get_pool();
        let now = Utc::now();
        
        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (id, chat_id, role, content, attachments, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(chat_id)
        .bind(role)
        .bind(content)
        .bind(attachments)
        .bind(now)
        .fetch_one(pool)
        .await?;
        
        Ok(message)
    }
    
    pub async fn get_by_chat_id(chat_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        let pool = get_pool();
        
        let messages = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE chat_id = ?1 ORDER BY created_at ASC"
        )
        .bind(chat_id)
        .fetch_all(pool)
        .await?;
        
        Ok(messages)
    }
    
    pub async fn get_by_id(id: &str) -> Result<Option<Self>, sqlx::Error> {
        let pool = get_pool();
        
        let message = sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE id = ?1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(message)
    }
    
    pub async fn update_content(id: &str, content: &str) -> Result<(), sqlx::Error> {
        let pool = get_pool();
        
        sqlx::query(
            "UPDATE messages SET content = ?1 WHERE id = ?2"
        )
        .bind(content)
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete(id: &str) -> Result<(), sqlx::Error> {
        let pool = get_pool();
        
        sqlx::query("DELETE FROM messages WHERE id = ?1")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
}
