use indexmap::IndexMap;
use std::path::Path;
use std::collections::HashMap;
use serde::Serialize;

use crate::blame::Blame;

#[derive(Debug, Serialize)]
pub struct LanguageItem {
    pub cases: HashMap<String, String>,
    pub version: String,
}

type LanguageJson = IndexMap<String, LanguageItem>;

/**
 * Process the english.txt file.
 *
 * This file is the "base" language, and defines what STR_ strings are
 * available, and when a translation is up-to-date.
 *
 * It can be based on older commits, as to find back what english string a
 * translation was translating.
 */
pub fn english(path: &Path, commit: &String) -> LanguageJson {
    let mut language_map = LanguageJson::new();

    let blame = Blame::new(path, &"english".to_string(), commit);
    let mut iter = blame.iter();

    while let Some(line) = iter.next() {
        /* English never has any cases, so no need to handled those scenarios. */
        language_map.insert(line.id, LanguageItem {
            cases: vec![(line.case, line.translation)].into_iter().collect(),
            version: line.commit,
        });
    }

    language_map
}

/**
 * Process any other language file besides english.txt.
 */
pub fn language(path: &Path, language: &String) -> LanguageJson {
    let mut language_map = LanguageJson::new();
    let mut english_maps: HashMap<String, LanguageJson> = HashMap::new();

    let blame = Blame::new(path, &language, &"master".to_string());
    let mut iter = blame.iter();

    while let Some(line) = iter.next() {
        /* For each line, check when the string in english.txt was last
         * changed when this translation commit was made. This is the
         * most likely string that was translated. */
        let english_map = english_maps.entry(line.commit.clone()).or_insert_with(|| {
            english(path, &line.commit)
        });
        if !english_map.contains_key(&line.id) {
            eprintln!("ERROR: Couldn't find version of English translation for {} at commit {}", line.id, line.commit);
            continue;
        }
        let english_version = english_map.get(&line.id).unwrap().version.clone();

        language_map.entry(line.id).or_insert_with(|| {
            LanguageItem {
                cases: HashMap::new(),
                version: english_version,
            }
        }).cases.insert(line.case, line.translation);
    }

    language_map
}
