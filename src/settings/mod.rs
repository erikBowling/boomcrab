use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize)]
pub struct BoomCrabSettings {
    pub sound_files_directory: String
}

impl BoomCrabSettings {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_or_create()
    }

    fn get_config_file_path() -> std::path::PathBuf {
        let config_directory = match dirs::config_dir() {
            Some(dir) => dir,
            None => {
                eprintln!("Failed to get config directory");
                panic!("Cannot continue without config directory");
            }
        };

        config_directory.join("boomcrab.toml")
    }

    /// Load settings from config file, or create default settings if file doesn't exist
    fn load_or_create() -> Result<BoomCrabSettings, Box<dyn std::error::Error>> {
        let config_file_path = Self::get_config_file_path();
        let config_path = Path::new(&config_file_path);
        
        if config_path.exists() {
            // Load existing settings
            let config_content = fs::read_to_string(&config_file_path)?;
            let settings: BoomCrabSettings = toml::from_str(&config_content)?;
            println!("Loaded settings from: {}", config_file_path.display());
            Ok(settings)
        } else {
            // Create default settings and config file
            let default_settings = Self::create_default_settings();
            
            // Create the parent directory if it doesn't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Serialize and write the default config file
            let toml_string = toml::to_string_pretty(&default_settings)?;
            fs::write(config_path, toml_string)?;
            println!("Created default config file at: {}", config_file_path.display());
            
            Ok(default_settings)
        }
    }

    fn create_default_settings() -> BoomCrabSettings {
        BoomCrabSettings {
            sound_files_directory: String::new()
        }
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_file_path = Self::get_config_file_path();
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(&config_file_path, toml_string)?;
        Ok(())
    }
}