use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".mybudy";
const CONFIG_FILE: &str = "config.json";
const MODELS_FILE: &str = "models.json";
const SOUL_FILE: &str = "soul.md";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub theme: String,
    pub language: String,
    pub float_button_position: Position,
    pub auto_start: bool,
    pub shortcuts: Shortcuts,
    pub voice_enabled: bool,
    pub default_model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shortcuts {
    pub show_window: String,
    pub voice_wake: String,
    pub screenshot: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: "dark".to_string(),
            language: "zh-CN".to_string(),
            float_button_position: Position { x: 0.0, y: 0.0 },
            auto_start: false,
            shortcuts: Shortcuts {
                show_window: "CmdOrCtrl+Shift+M".to_string(),
                voice_wake: "CmdOrCtrl+Shift+V".to_string(),
                screenshot: "CmdOrCtrl+Shift+S".to_string(),
            },
            voice_enabled: true,
            default_model: "kimi".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub api_key: String,
    pub api_url: String,
    pub models: Vec<String>,
    pub default_model: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelsConfig {
    pub providers: Vec<ModelConfig>,
}

impl Default for ModelsConfig {
    fn default() -> Self {
        ModelsConfig {
            providers: vec![
                ModelConfig {
                    id: "kimi".to_string(),
                    name: "Kimi".to_string(),
                    provider: "kimi".to_string(),
                    api_key: "".to_string(),
                    api_url: "https://api.moonshot.cn/v1".to_string(),
                    models: vec!["moonshot-v1-8k".to_string(), "moonshot-v1-32k".to_string(), "moonshot-v1-128k".to_string()],
                    default_model: "moonshot-v1-8k".to_string(),
                    enabled: true,
                },
                ModelConfig {
                    id: "openai".to_string(),
                    name: "OpenAI".to_string(),
                    provider: "openai".to_string(),
                    api_key: "".to_string(),
                    api_url: "https://api.openai.com/v1".to_string(),
                    models: vec!["gpt-4".to_string(), "gpt-4-turbo".to_string(), "gpt-3.5-turbo".to_string()],
                    default_model: "gpt-3.5-turbo".to_string(),
                    enabled: false,
                },
                ModelConfig {
                    id: "deepseek".to_string(),
                    name: "DeepSeek".to_string(),
                    provider: "deepseek".to_string(),
                    api_key: "".to_string(),
                    api_url: "https://api.deepseek.com/v1".to_string(),
                    models: vec!["deepseek-chat".to_string(), "deepseek-coder".to_string()],
                    default_model: "deepseek-chat".to_string(),
                    enabled: false,
                },
            ],
        }
    }
}

fn get_config_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(CONFIG_DIR)
}

pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_config_dir();
    
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    // Create default config if not exists
    let config_path = config_dir.join(CONFIG_FILE);
    if !config_path.exists() {
        let config = Config::default();
        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(&config_path, config_json)?;
    }

    // Create default models config if not exists
    let models_path = config_dir.join(MODELS_FILE);
    if !models_path.exists() {
        let models = ModelsConfig::default();
        let models_json = serde_json::to_string_pretty(&models)?;
        fs::write(&models_path, models_json)?;
    }

    // Create default soul.md if not exists
    let soul_path = config_dir.join(SOUL_FILE);
    if !soul_path.exists() {
        let soul_content = r#"# MyBudy Soul

You are MyBudy, a helpful AI assistant.

## Personality
- Friendly and approachable
- Professional yet casual
- Always eager to help

## Capabilities
- Answer questions
- Help with coding
- Read and analyze files
- Capture screenshots
- Control system applications

## Guidelines
- Be concise but thorough
- Ask clarifying questions when needed
- Respect user privacy
- Provide accurate information
"#;
        fs::write(&soul_path, soul_content)?;
    }

    Ok(())
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_dir().join(CONFIG_FILE);
    let config_json = fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&config_json)?;
    Ok(config)
}

pub fn set_config(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_dir().join(CONFIG_FILE);
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;
    Ok(())
}

pub fn get_models() -> Result<ModelsConfig, Box<dyn std::error::Error>> {
    let models_path = get_config_dir().join(MODELS_FILE);
    let models_json = fs::read_to_string(&models_path)?;
    let models: ModelsConfig = serde_json::from_str(&models_json)?;
    Ok(models)
}

pub fn set_models(models: ModelsConfig) -> Result<(), Box<dyn std::error::Error>> {
    let models_path = get_config_dir().join(MODELS_FILE);
    let models_json = serde_json::to_string_pretty(&models)?;
    fs::write(&models_path, models_json)?;
    Ok(())
}

pub fn get_soul() -> Result<String, Box<dyn std::error::Error>> {
    let soul_path = get_config_dir().join(SOUL_FILE);
    let soul_content = fs::read_to_string(&soul_path)?;
    Ok(soul_content)
}
