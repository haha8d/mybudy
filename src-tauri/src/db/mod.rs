use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{AppHandle, Manager};
use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub mod chat;
pub mod message;

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

pub async fn init_database(_app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Windows 上使用文档目录，避免权限问题
    let app_dir = dirs::document_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("MyBudy");
    
    eprintln!("Creating app directory: {}", app_dir.display());
    
    // 确保目录存在，如果不存在则创建
    match std::fs::create_dir_all(&app_dir) {
        Ok(_) => eprintln!("Directory created or already exists"),
        Err(e) => {
            eprintln!("Failed to create directory: {}, using temp dir", e);
            let temp_dir = std::env::temp_dir().join("MyBudy");
            std::fs::create_dir_all(&temp_dir)?;
            return init_db_in_dir(temp_dir).await;
        }
    }
    
    init_db_in_dir(app_dir).await
}

async fn init_db_in_dir(app_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = app_dir.join("mybudy.db");
    
    eprintln!("Database file path: {}", db_path.display());
    
    // 使用简单的相对路径格式
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    eprintln!("Connecting to: {}", db_url);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;
    
    eprintln!("Connected successfully, creating tables...");
    
    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS chats (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            model_provider TEXT NOT NULL,
            model_name TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            chat_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            attachments TEXT,
            created_at DATETIME NOT NULL,
            FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(&pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_messages_chat_id ON messages(chat_id)"
    )
    .execute(&pool)
    .await?;
    
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_chats_updated_at ON chats(updated_at DESC)"
    )
    .execute(&pool)
    .await?;
    
    POOL.set(pool).expect("Failed to set database pool");
    
    eprintln!("Database initialized successfully");
    
    Ok(())
}

pub fn get_pool() -> &'static Pool<Sqlite> {
    POOL.get().expect("Database pool not initialized")
}
