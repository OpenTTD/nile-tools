use std::path::PathBuf;

use nile_library::validate::{validate_translation, Dialect, LanguageConfig};

use crate::{config, types::LanguageJson};

pub fn validate(config_path: &PathBuf, data_path: &PathBuf, project: &String, language: &String) {
    let base_filename = data_path.join(project).join("base.json");
    let language_filename = data_path.join(project).join(format!("{}.json", language));

    let base_json = std::fs::read_to_string(&base_filename).unwrap();
    let base_json: LanguageJson = serde_json::from_str(&base_json).unwrap();

    let language_json = std::fs::read_to_string(&language_filename).unwrap();
    let language_json: LanguageJson = serde_json::from_str(&language_json).unwrap();

    let config_language = config::read_config_language(config_path, language);
    let config_plurals = config::read_config_plurals(config_path);

    let language_config: LanguageConfig = LanguageConfig {
        dialect: Dialect::OPENTTD, // TODO read from project
        cases: config_language.case.unwrap_or_default(),
        genders: config_language.gender.unwrap_or_default(),
        plural_count: config_plurals
            .get(&config_language.plural.to_string())
            .unwrap()
            .len(),
    };

    language_json
        .iter()
        .for_each(|(id, item)| match base_json.get(id) {
            None => {
                eprintln!("ERROR: String {} is missing in base language", id);
            }
            Some(base_item) => {
                let base_translation = base_item.cases.get("default").unwrap();

                item.cases.iter().for_each(|(case, translation)| {
                    let result =
                        validate_translation(&language_config, base_translation, case, translation);
                    if !result.errors.is_empty() {
                        eprintln!(
                            "ERROR: Translation for {}.{} is invalid: {:?}",
                            id, case, result.errors
                        );
                    }
                });
            }
        });
}
