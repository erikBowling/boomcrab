use std::{fmt, fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug)]
pub enum SettingsError {
    ConfigDirNotFound,
    FileRead(std::io::Error),
    ParseError(toml::de::Error),
    SerializeError(toml::ser::Error),
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingsError::ConfigDirNotFound => write!(f, "Could not find config directory"),
            SettingsError::FileRead(e) => write!(f, "Failed to read settings file: {}", e),
            SettingsError::ParseError(e) => write!(f, "Failed to parse settings: {}", e),
            SettingsError::SerializeError(e) => write!(f, "Failed to serialize settings: {}", e),
        }
    }
}

impl std::error::Error for SettingsError {}

impl From<std::io::Error> for SettingsError {
    fn from(err: std::io::Error) -> Self {
        SettingsError::FileRead(err)
    }
}

impl From<toml::de::Error> for SettingsError {
    fn from(err: toml::de::Error) -> Self {
        SettingsError::ParseError(err)
    }
}

impl From<toml::ser::Error> for SettingsError {
    fn from(err: toml::ser::Error) -> Self {
        SettingsError::SerializeError(err)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BoomCrabSettings {
    pub sound_files_directory: String,
}

impl Default for BoomCrabSettings {
    fn default() -> Self {
        Self {
            sound_files_directory: String::new(),
        }
    }
}

impl BoomCrabSettings {
    pub fn new() -> Self {
        Self::new_from_file().unwrap_or_else(|e| {
            eprintln!("Warning: Could not load settings ({}). Using defaults.", e);
            Self::default()
        })
    }

    fn new_from_file() -> Result<Self, SettingsError> {
        let settings_file_path = Self::get_settings_file_path()?;
        let settings_toml_string = std::fs::read_to_string(settings_file_path)?;
        Ok(toml::from_str(&settings_toml_string)?)
    }

    fn get_settings_file_path() -> Result<PathBuf, SettingsError> {
        dirs::config_local_dir()
            .map(|dir| dir.join("boomcrab.toml"))
            .ok_or(SettingsError::ConfigDirNotFound)
    }

    pub fn save_to_file(&self) -> Result<(), SettingsError> {
        let config_file_path = Self::get_settings_file_path()?;
        let str_toml = toml::to_string_pretty(self)?;
        fs::write(&config_file_path, str_toml)?;
        Ok(())
    }

    fn settings_file_exists() -> bool {
        Self::get_settings_file_path()
            .and_then(|p| p.try_exists().map_err(Into::into))
            .unwrap_or(false)
    }
}
