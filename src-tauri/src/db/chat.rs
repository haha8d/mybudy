use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use crate::db::get_pool;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Chat {
    pub id: String,
    pub title: String,
    pub model_provider: String,
    pub model_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Chat {
    pub async fn create(
        id: &str,
        title: &str,
        model_provider: &str,
        model_name: &str,
    ) -> Result<Self, sqlx::Error> {
        let pool = get_pool();
        let now = Utc::now();
        
        let chat = sqlx::query_as::<_, Chat>(
            r#"
            INSERT INTO chats (id, title, model_provider, model_name, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            RETURNING *
            "#
        )
        .bind(id)
        .bind(title)
        .bind(model_provider)
        .bind(model_name)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;
        
        Ok(chat)
    }
    
    pub async fn get_all() -> Result<Vec<Self>, sqlx::Error> {
        let pool = get_pool();
        
        let chats = sqlx::query_as::<_, Chat>(
            "SELECT * FROM chats ORDER BY updated_at DESC"
        )
        .fetch_all(pool)
        .await?;
        
        Ok(chats)
    }
    
    pub async fn get_by_id(id: &str) -> Result<Option<Self>, sqlx::Error> {
        let pool = get_pool();
        
        let chat = sqlx::query_as::<_, Chat>(
            "SELECT * FROM chats WHERE id = ?1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        
        Ok(chat)
    }
    
    pub async fn update_title(id: &str, title: &str) -> Result<(), sqlx::Error> {
        let pool = get_pool();
        let now = Utc::now();
        
        sqlx::query(
            "UPDATE chats SET title = ?1, updated_at = ?2 WHERE id = ?3"
        )
        .bind(title)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete(id: &str) -> Result<(), sqlx::Error> {
        let pool = get_pool();
        
        sqlx::query("DELETE FROM messages WHERE chat_id = ?1")
            .bind(id)
            .execute(pool)
            .await?;
        
        sqlx::query("DELETE FROM chats WHERE id = ?1")
            .bind(id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn update_timestamp(id: &str) -> Result<(), sqlx::Error> {
        let pool = get_pool();
        let now = Utc::now();
        
        sqlx::query(
            "UPDATE chats SET updated_at = ?1 WHERE id = ?2"
        )
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}
