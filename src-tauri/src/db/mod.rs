use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{AppHandle, Manager};
use once_cell::sync::OnceCell;

pub mod chat;
pub mod message;

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

pub async fn init_database(app: &AppHandle) -> Result<(), sqlx::Error> {
    let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    
    let db_path = app_dir.join("mybudy.db");
    let db_url = format!("sqlite:{}", db_path.to_str().unwrap());
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    
    // Create tables manually since we don't have migrations set up
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
    
    Ok(())
}

pub fn get_pool() -> &'static Pool<Sqlite> {
    POOL.get().expect("Database pool not initialized")
}
