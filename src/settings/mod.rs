use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

// #[derive(Serialize, Deserialize)]
pub struct BoomCrabSettings {
    pub sound_files_directory: String,
}
