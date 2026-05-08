use std::{
    env::home_dir,
    path::{Component, Path, PathBuf},
};

use crate::{
    core::data_struct::StsMod::StsMod,
    error::{RoxyError, RoxyResult},
};

#[derive(Debug)]
pub struct Profile<'a> {
    pub name: String,
    pub data: ProfileData<'a>,
    pub enabled_mod: Vec<StsMod>,
    pub version_sts: String,
}

#[derive(Debug)]
pub struct ProfileData<'a> {
    pub icon_uri: &'a Path,
    pub exe_uri: &'a Path,
}

impl<'a> From<String> for Profile<'a> {
    fn from(value: String) -> Self {
        Self {
            name: value,
            data: ProfileData {
                icon_uri: Path::new(""),
                exe_uri: Path::new(""),
            },
            enabled_mod: Vec::new(),
            version_sts: String::new(),
        }
    }
}

impl<'a> Profile<'a> {
    pub fn path(&self) -> PathBuf {
        profile_root().join(&self.name)
    }

    pub(crate) fn ensure_valid_name(&self) -> RoxyResult {
        if is_valid_profile_name(&self.name) {
            Ok(())
        } else {
            Err(RoxyError::InvalidProfileName)
        }
    }
}

pub(crate) fn profile_root() -> PathBuf {
    home_dir()
        .expect("HOME is not set")
        .join("/.local/share/roxy")
}

fn is_valid_profile_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut components = Path::new(name).components();
    matches!(components.next(), Some(Component::Normal(_))) && components.next().is_none()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn profile_path_is_nested_under_profile_root() {
        let profile = Profile::from("test-profile".to_string());
        assert_eq!(profile.path(), profile_root().join("test-profile"));
    }

    #[test]
    fn profile_root_has_expected_suffix() {
        assert!(profile_root().ends_with(Path::new(".roxybestgirl")));
    }

    #[test]
    fn empty_profile_name_is_rejected() {
        let profile = Profile::from(String::new());

        assert!(matches!(
            profile.ensure_valid_name(),
            Err(RoxyError::InvalidProfileName)
        ));
    }

    #[test]
    fn path_traversal_profile_name_is_rejected() {
        let profile = Profile::from("../escape".to_string());

        assert!(matches!(
            profile.ensure_valid_name(),
            Err(RoxyError::InvalidProfileName)
        ));
    }
}
