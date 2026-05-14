use std::{
    collections::HashMap,
    env::home_dir,
    io,
    path::{Path, PathBuf},
};

use crate::language::{default::DEFAULT_EN_LANG_MAP, element::LANGUAGE_ELEMENT};

pub mod boot;
pub mod default;
pub mod element;
pub mod get;
pub mod load;
pub mod save;

pub struct LangHelper {
    pub current_lang: String,
    pub all_lang_resouce: HashMap<String, HashMap<&'static str, String>>,
    pub lang_dir: PathBuf,
}

impl Default for LangHelper {
    fn default() -> Self {
        let mut default_map = HashMap::new();

        if let Ok(readable_en_map) = DEFAULT_EN_LANG_MAP.read() {
            for (k, v) in readable_en_map.iter() {
                default_map.entry(*k).or_insert(v.to_string());
            }
        } else {
            for v in LANGUAGE_ELEMENT.iter() {
                default_map.entry(v).or_insert(v.to_string());
            }
        }
        Self {
            current_lang: "en_us".to_string(),
            all_lang_resouce: HashMap::from([("en_us".to_string(), default_map)]),
            lang_dir: home_dir()
                .unwrap_or(PathBuf::new().join("/"))
                .join(".local/share/lang"),
        }
    }
}
