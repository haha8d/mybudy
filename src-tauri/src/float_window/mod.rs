use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};

const FLOAT_WINDOW_SIZE: f64 = 64.0;

pub fn create_float_window(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let window = WebviewWindowBuilder::new(
        app,
        "float",
        WebviewUrl::App("float.html".into())
    )
    .title("")
    .inner_size(FLOAT_WINDOW_SIZE, FLOAT_WINDOW_SIZE)
    .min_inner_size(FLOAT_WINDOW_SIZE, FLOAT_WINDOW_SIZE)
    .max_inner_size(FLOAT_WINDOW_SIZE, FLOAT_WINDOW_SIZE)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .transparent(true)
    .resizable(false)
    .visible(true)
    .build()?;

    // Position at bottom-right corner
    let monitor = window.current_monitor()?.ok_or("No monitor found")?;
    let monitor_size = monitor.size();
    let monitor_position = monitor.position();
    
    let x = monitor_position.x as f64 + monitor_size.width as f64 - FLOAT_WINDOW_SIZE - 20.0;
    let y = monitor_position.y as f64 + monitor_size.height as f64 - FLOAT_WINDOW_SIZE - 80.0;
    
    window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }))?;

    // Make window draggable
    window.set_ignore_cursor_events(false)?;

    Ok(())
}

pub fn create_tray_icon(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let toggle_i = MenuItem::with_id(app, "toggle_float", "Hide Float Button", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    
    let menu = Menu::with_items(app, &[
        &show_i,
        &toggle_i,
        &separator1,
        &settings_i,
        &separator2,
        &quit_i,
    ])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "show" => {
                    show_main_window(app);
                }
                "toggle_float" => {
                    toggle_float_window(app);
                }
                "settings" => {
                    show_main_window(app);
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button: MouseButton::Left, .. } = event {
                let app = tray.app_handle();
                show_main_window(app);
            }
        })
        .build(app)?;

    Ok(())
}

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        // Create main window if it doesn't exist
        let _ = WebviewWindowBuilder::new(
            app,
            "main",
            WebviewUrl::App("/".into())
        )
        .title("MyBudy")
        .inner_size(900.0, 700.0)
        .min_inner_size(400.0, 500.0)
        .decorations(true)
        .always_on_top(false)
        .visible(true)
        .build();
    }
}

pub fn toggle_float_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("float") {
        let is_visible = window.is_visible().unwrap_or(true);
        if is_visible {
            let _ = window.hide();
        } else {
            let _ = window.show();
        }
    }
}

pub fn hide_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}
