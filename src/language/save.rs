use std::io;

use crate::language::{LangHelper, default::DEFAULT_EN_LANG_MAP, element::LANGUAGE_ELEMENT};

impl LangHelper {
    // use to save the default english config as en_Us.yml
    pub async fn save_default(&mut self) -> io::Result<()> {
        tokio::fs::create_dir_all(&self.lang_dir).await?;

        let readable_default_map = {
            DEFAULT_EN_LANG_MAP
                .read()
                .map_err(|_| io::Error::other("failed to read default english language map"))?
        };

        let mut output = std::collections::BTreeMap::<String, String>::new();

        for key in LANGUAGE_ELEMENT.iter() {
            let value = readable_default_map
                .get(key)
                .cloned()
                .unwrap_or_else(|| (*key).to_string());
            output.insert((*key).to_string(), value);
        }

        let yaml = serde_yaml::to_string(&output).map_err(|err| {
            io::Error::other(format!("failed to serialize default language: {err}"))
        })?;

        let output_path = self.lang_dir.join("en_Us.yml");
        tokio::fs::write(output_path, yaml).await
    }

    pub async fn try_save_default(&mut self) -> io::Result<()> {
        if self.lang_dir.join("en_Us.yml").exists() {
            return Ok(());
        }
        self.save_default().await
    }
}
