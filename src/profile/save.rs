use std::{fs, path::Path};

use tracing::error;

use crate::{
    error::{RoxyError, RoxyResult},
    profile::Profile,
};

impl Profile {
    pub fn save(&self, path: &Path) -> RoxyResult<()> {
        match toml::to_string(self) {
            Ok(contents) => match fs::write(path, contents) {
                Ok(_) => Ok(()),
                Err(e) => Err(RoxyError::IOError(e.to_string())),
            },
            Err(e) => Err(RoxyError::ProfileParseError(e.to_string())),
        }
    }

    pub fn save_from_name(&self) -> RoxyResult<()> {
        let path = Self::profile_root().join(&self.name);
        self.save(path.as_path())
    }

    pub fn save_all(contents: &Vec<Profile>) -> RoxyResult<()> {
        let root = Self::profile_root();

        for c in contents {
            let save_path_dir = root.join(&c.name);
            if !save_path_dir.exists() {
                if let Err(e) = fs::create_dir(&save_path_dir) {
                    error!(
                        "cant create dir at {} while saving profile!: {}",
                        &save_path_dir.as_os_str().to_string_lossy(),
                        e
                    );
                }
            }
            let save_path = save_path_dir.join("profile.toml");

            if let Ok(toml_string) = toml::to_string(c) {
                if let Err(e) = fs::write(&save_path, toml_string) {
                    error!(
                        "Error on write profile.toml to {}: {}",
                        save_path.to_string_lossy(),
                        e
                    );
                }
            }
        }
        Ok(())
    }
}
