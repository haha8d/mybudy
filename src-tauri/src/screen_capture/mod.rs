use screenshots::Screen;
use std::fs;
use std::path::PathBuf;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub fn capture_screen() -> Result<String, Box<dyn std::error::Error>> {
    let screens = Screen::all()?;
    
    if let Some(screen) = screens.first() {
        let image = screen.capture()?;
        
        // Save to temp file
        let temp_dir = std::env::temp_dir();
        let filename = format!("mybudy_screenshot_{}.png", chrono::Utc::now().timestamp());
        let filepath = temp_dir.join(&filename);
        
        image.save(&filepath)?;
        
        // Convert to base64
        let bytes = fs::read(&filepath)?;
        let base64_string = BASE64.encode(&bytes);
        
        // Clean up temp file
        let _ = fs::remove_file(&filepath);
        
        Ok(format!("data:image/png;base64,{}", base64_string))
    } else {
        Err("No screen found".into())
    }
}

pub fn capture_screen_area(x: i32, y: i32, width: u32, height: u32) -> Result<String, Box<dyn std::error::Error>> {
    let screens = Screen::all()?;
    
    if let Some(screen) = screens.first() {
        let image = screen.capture_area(x, y, width, height)?;
        
        let temp_dir = std::env::temp_dir();
        let filename = format!("mybudy_screenshot_{}.png", chrono::Utc::now().timestamp());
        let filepath = temp_dir.join(&filename);
        
        image.save(&filepath)?;
        
        let bytes = fs::read(&filepath)?;
        let base64_string = BASE64.encode(&bytes);
        
        let _ = fs::remove_file(&filepath);
        
        Ok(format!("data:image/png;base64,{}", base64_string))
    } else {
        Err("No screen found".into())
    }
}
