use std::path::PathBuf;

use crate::types::{ConfigLanguage, ConfigPlurals};

pub fn read_config_language(path: &PathBuf, language: &String) -> ConfigLanguage {
    let filename = path.join("languages").join(format!("{}.json", language));
    let json = std::fs::read_to_string(&filename).unwrap();
    let config: ConfigLanguage = serde_json::from_str(&json).unwrap();
    config
}

pub fn read_config_plurals(path: &PathBuf) -> ConfigPlurals {
    let filename = path.join("plurals.json");
    let json = std::fs::read_to_string(&filename).unwrap();
    let plurals: ConfigPlurals = serde_json::from_str(&json).unwrap();
    plurals
}
