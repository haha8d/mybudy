use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

pub mod chat;
pub mod message;

static CONN: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn init_database() -> Result<(), Box<dyn std::error::Error>> {
    // Windows 上使用文档目录
    let app_dir = dirs::document_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("MyBudy");
    
    eprintln!("Creating app directory: {}", app_dir.display());
    
    // 创建目录
    std::fs::create_dir_all(&app_dir)?;
    
    let db_path = app_dir.join("mybudy.db");
    eprintln!("Database path: {}", db_path.display());
    
    // 打开或创建数据库
    let conn = Connection::open(&db_path)?;
    
    eprintln!("Database opened successfully");
    
    // 创建表
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS chats (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            model_provider TEXT NOT NULL,
            model_name TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#,
        [],
    )?;
    
    conn.execute(
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
        "#,
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_messages_chat_id ON messages(chat_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_chats_updated_at ON chats(updated_at DESC)",
        [],
    )?;
    
    CONN.set(Mutex::new(conn)).expect("Failed to set database connection");
    
    eprintln!("Database initialized successfully");
    
    Ok(())
}

pub fn get_conn() -> std::sync::MutexGuard<'static, Connection> {
    CONN.get().expect("Database not initialized").lock().expect("Failed to lock database")
}
