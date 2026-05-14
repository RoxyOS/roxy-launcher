use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use tracing::{error, info};

use crate::{
    language::{LangHelper, default::DEFAULT_EN_LANG_MAP, element::LANGUAGE_ELEMENT},
    utils::path_util::expand_home_dir_string,
};

pub static DEFAULT_LANG_DIR: &str = "~/.local/share/roxy/lang";

impl LangHelper {
    pub async fn load_lang_from_path(&mut self, path: &Path) -> io::Result<()> {
        let content = tokio::fs::read_to_string(path).await?;
        let yaml = serde_yaml::from_str::<serde_yaml::Value>(&content).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("failed to parse language file {}: {error}", path.display()),
            )
        })?;

        let default_lang = DEFAULT_EN_LANG_MAP
            .read()
            .map_err(|_| io::Error::other("failed to read default language map"))?;

        let mut lang_map = HashMap::new();

        for key in LANGUAGE_ELEMENT.iter() {
            let value = yaml
                .get(*key)
                .and_then(serde_yaml::Value::as_str)
                .map(ToOwned::to_owned)
                .or_else(|| default_lang.get(key).cloned())
                .unwrap_or_else(|| (*key).to_string());

            lang_map.insert(*key, value);
        }
        if let Some(file_name) = path.file_name() {
            let file_name = file_name.to_string_lossy().to_string();
            self.all_lang_resouce
                .entry(file_name)
                .insert_entry(lang_map);
        }

        Ok(())
    }

    pub async fn load_lang_from_name(&mut self, name: &str) -> io::Result<()> {
        let path = self.lang_dir.join(format!("{}.yml", name));
        self.load_lang_from_path(&path).await?;
        let path = self.lang_dir.join(format!("{}.yaml", name));

        self.load_lang_from_path(&path).await?;
        Ok(())
    }

    pub async fn load_all(&mut self) -> io::Result<()> {
        if let Ok(readdir) = fs::read_dir(&self.lang_dir) {
            for entry in readdir.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                match self.load_lang_from_name(&name).await {
                    Ok(_) => {
                        info!("load language {}", name);
                    }
                    Err(e) => {
                        error!("falid on loading language {}: {}\n skip it", name, e)
                    }
                }
            }
        }
        Ok(())
    }
}
