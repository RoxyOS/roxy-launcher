use std::{
    env::home_dir,
    path::{Component, Path, PathBuf},
};

use crate::error::{RoxyError, RoxyResult};

pub struct Profile(pub String);

impl From<String> for Profile {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Profile {
    pub fn path(&self) -> PathBuf {
        profile_root().join(&self.0)
    }

    pub(crate) fn ensure_valid_name(&self) -> RoxyResult {
        if is_valid_profile_name(&self.0) {
            Ok(())
        } else {
            Err(RoxyError::InvalidProfileName)
        }
    }
}

pub(crate) fn profile_root() -> PathBuf {
    home_dir().expect("HOME is not set").join(".roxybestgirl")
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
        let profile = Profile("test-profile".into());

        assert_eq!(profile.path(), profile_root().join("test-profile"));
    }

    #[test]
    fn profile_root_has_expected_suffix() {
        assert!(profile_root().ends_with(Path::new(".roxybestgirl")));
    }

    #[test]
    fn empty_profile_name_is_rejected() {
        let profile = Profile(String::new());

        assert!(matches!(
            profile.ensure_valid_name(),
            Err(RoxyError::InvalidProfileName)
        ));
    }

    #[test]
    fn path_traversal_profile_name_is_rejected() {
        let profile = Profile("../escape".into());

        assert!(matches!(
            profile.ensure_valid_name(),
            Err(RoxyError::InvalidProfileName)
        ));
    }

    #[test]
    fn nested_profile_name_is_rejected() {
        let profile = Profile("nested/profile".into());

        assert!(matches!(
            profile.ensure_valid_name(),
            Err(RoxyError::InvalidProfileName)
        ));
    }

    #[test]
    fn simple_profile_name_is_accepted() {
        let profile = Profile("test-profile".into());

        assert!(profile.ensure_valid_name().is_ok());
    }
}
