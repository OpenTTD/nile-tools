use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type LanguageJson = IndexMap<String, LanguageItem>;
pub type ConfigPlurals = HashMap<String, Vec<String>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageItem {
    pub cases: HashMap<String, String>,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigLanguage {
    pub name: String,
    pub ownname: String,
    pub isocode: String,
    pub plural: usize,
    pub textdir: String,
    pub digitsep: String,
    pub digitsepcur: String,
    pub decimalsep: String,
    pub winlangid: usize,
    pub grflangid: usize,
    pub gender: Option<Vec<String>>,
    pub case: Option<Vec<String>>,
}
