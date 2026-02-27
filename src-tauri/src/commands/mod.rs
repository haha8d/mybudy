use tauri::AppHandle;
use crate::config::{get_config as config_get_config, set_config as config_set_config, Config, get_models, set_models, ModelsConfig};
use crate::db::chat::Chat;
use crate::db::message::Message;
use crate::screen_capture;
use crate::voice;
use std::fs;
use uuid::Uuid;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_file(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_directory(path: &str) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                result.push(name.to_string());
            }
        }
    }
    
    Ok(result)
}

#[tauri::command]
pub fn capture_screen() -> Result<String, String> {
    screen_capture::capture_screen().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_application(app_name: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-a")
            .arg(app_name)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args([&"/C",&"start",&"",app_name])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new(app_name)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub fn get_config() -> Result<Config, String> {
    config_get_config().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_config(config: Config) -> Result<(), String> {
    config_set_config(config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_models_cmd() -> Result<ModelsConfig, String> {
    get_models().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_models_cmd(models: ModelsConfig) -> Result<(), String> {
    set_models(models).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_chat(title: &str, model_provider: &str, model_name: &str) -> Result<Chat, String> {
    let id = Uuid::new_v4().to_string();
    Chat::create(&id, title, model_provider, model_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_chats() -> Result<Vec<Chat>, String> {
    Chat::get_all().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_messages(chat_id: &str) -> Result<Vec<Message>, String> {
    Message::get_by_chat_id(chat_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_message(
    chat_id: &str,
    role: &str,
    content: &str,
    attachments: Option<&str>,
) -> Result<Message, String> {
    let id = Uuid::new_v4().to_string();
    let message = Message::create(&id, chat_id, role, content, attachments)
        .await
        .map_err(|e| e.to_string())?;
    
    // Update chat timestamp
    let _ = Chat::update_timestamp(chat_id).await;
    
    Ok(message)
}

#[tauri::command]
pub async fn delete_chat(chat_id: &str) -> Result<(), String> {
    Chat::delete(chat_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn start_voice_recognition() -> Result<String, String> {
    voice::start_voice_recognition().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_voice_recognition() -> Result<String, String> {
    voice::stop_voice_recognition().map_err(|e| e.to_string())
}

// Window control commands
#[tauri::command]
pub fn show_main_window(app: AppHandle) {
    crate::float_window::show_main_window(&app);
}

#[tauri::command]
pub fn hide_main_window(app: AppHandle) {
    crate::float_window::hide_main_window(&app);
}

#[tauri::command]
pub fn toggle_float_window(app: AppHandle) {
    crate::float_window::toggle_float_window(&app);
}
