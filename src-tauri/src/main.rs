// use tauri::Manager;

mod commands;
mod config;
mod db;
mod float_window;
mod models;
mod screen_capture;
mod voice;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // Note: global-shortcut plugin in Tauri V2 doesn't use init()
        // Shortcuts are registered at runtime using the JavaScript API
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle();
            tauri::async_runtime::block_on(async {
                if let Err(e) = db::init_database(&app_handle).await {
                    eprintln!("Failed to initialize database: {}", e);
                    // 数据库初始化失败不阻止应用启动
                }
            });

            // Initialize config
            if let Err(e) = config::init_config() {
                eprintln!("Failed to initialize config: {}", e);
            }

            // Create floating button window
            if let Err(e) = float_window::create_float_window(&app_handle) {
                eprintln!("Failed to create float window: {}", e);
            }

            // Create tray icon
            if let Err(e) = float_window::create_tray_icon(&app_handle) {
                eprintln!("Failed to create tray icon: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::read_file,
            commands::write_file,
            commands::list_directory,
            commands::capture_screen,
            commands::open_application,
            commands::get_config,
            commands::set_config,
            commands::get_models_cmd,
            commands::set_models_cmd,
            commands::create_chat,
            commands::get_chats,
            commands::get_messages,
            commands::send_message,
            commands::delete_chat,
            commands::start_voice_recognition,
            commands::stop_voice_recognition,
            commands::show_main_window,
            commands::hide_main_window,
            commands::toggle_float_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
