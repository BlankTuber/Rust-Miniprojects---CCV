use anyhow::{Context, Error, Result};
use log::{LevelFilter, debug, info, warn};
use serde::{Deserialize, Serialize};
use simplelog::{Config, SimpleLogger};
use std::{
    env::current_exe,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    username: String,
    age: u16,
    favorite_num: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            username: String::from("Blank"),
            age: 20,
            favorite_num: 13,
        }
    }
}

fn get_current_path() -> Result<PathBuf> {
    let mut path_buffer = current_exe().context("Failed to get executable path")?;
    path_buffer.pop();
    path_buffer.push("settings.toml");
    Ok(path_buffer)
}

fn load_file(path: &Path) -> Result<Settings> {
    if !path.exists() {
        warn!("File not found! Generating default file.");
        return Ok(Settings::default());
    }

    let contents = read_to_string(path).with_context(|| format!("Failed to read {:?}", path))?;

    match toml::from_str::<Settings>(&contents) {
        Ok(settings) => {
            debug!("Settings loaded successfully");
            Ok(settings)
        }
        Err(e) => {
            warn!(
                "Settings file is corrupt or missing fields: {}. Overwriting using defaults.",
                e
            );
            Ok(Settings::default())
        }
    }
}

fn save_file(path: &Path, settings: &Settings) -> Result<(), Error> {
    let toml_string =
        toml::to_string_pretty(&settings).context("Failed to convert to serialize to TOML")?;

    write(path, toml_string).with_context(|| format!("Failed to write settings to {:?}", path))?;

    debug!("Settings saved successfully. {:?}", settings);
    Ok(())
}

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let settings_path = get_current_path().context("Error finding folder path")?;
    info!("Settings Path: {:?}", settings_path);

    let mut settings = load_file(&settings_path).context("Failed to generate or read settings")?;
    info!("Settings: {:?}", settings);

    settings.age = 20;
    settings.favorite_num = -13;

    match save_file(&settings_path, &settings) {
        Ok(()) => println!("Task completed."),
        Err(e) => println!("Error saving file: {e}"),
    };

    Ok(())
}
