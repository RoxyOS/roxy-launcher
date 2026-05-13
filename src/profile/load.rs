use std::{fs, path::Path};

use tracing::error;

use crate::{
    error::{RoxyError, RoxyResult},
    profile::{Profile, load},
};

impl Profile {
    pub fn load(path: &Path) -> RoxyResult<Self> {
        match fs::read_to_string(path.join("profile.toml")) {
            Ok(content) => {
                let result = match toml::from_str(&content) {
                    Ok(ok) => ok,
                    Err(e) => return Err(RoxyError::ProfileParseError(e.to_string())),
                };

                Ok(result)
            }
            Err(e) => Err(RoxyError::IOError(e.to_string())),
        }
    }

    pub fn load_from_name(name: &str) -> RoxyResult<Self> {
        let path = Self::profile_root().join(name);
        Self::load(path.as_path())
    }

    pub fn load_all() -> RoxyResult<Vec<Self>> {
        let root = Self::profile_root();
        let mut result = vec![];

        if let Ok(readdir) = fs::read_dir(root) {
            for entry in readdir {
                match entry {
                    Err(e) => {
                        error!("failed on reading dir:{}", e);
                        continue;
                    }
                    Ok(ok) => match Self::load(ok.path().as_path()) {
                        Ok(profile) => {
                            result.push(profile);
                        }
                        Err(e) => {
                            error!(
                                "failed on parse profile {}: {} \nskip it",
                                ok.file_name().to_string_lossy(),
                                e
                            );
                        }
                    },
                }
            }
        }
        Ok(result)
    }
}
