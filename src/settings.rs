use crate::prelude::*;
use serde::{Serialize, Deserialize, Deserializer};

const SETTINGS_FILE: &str = "settings.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipSettings {
    pub size: f32,
    pub force: f32,
    pub torque: f32,
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Color,
}

#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub ship: ShipSettings,
    #[serde(deserialize_with = "deserialize_color")]
    pub back_color: Color,
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error> 
where D: Deserializer<'de> {
    let hex = String::deserialize(deserializer)?;
    
    Color::hex(hex).map_err(serde::de::Error::custom)
}

fn read_config() -> std::io::Result<Settings> {
    let content = std::fs::read_to_string(SETTINGS_FILE)?;
    Ok(toml::from_str(&content)?)
}

impl Default for Settings {
    fn default() -> Self {
        let config = read_config();
        match config {
            Ok(settings) => settings,
            Err(error)   => panic!("Could not read settings.toml: {:?}", error),
        }
    }
}