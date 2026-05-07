use std::{env::home_dir, path::PathBuf};

use tap::Tap;

pub struct Profile(pub String);

impl From<String> for Profile {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Profile {
    pub fn path(&self) -> PathBuf {
        home_dir().unwrap().join(".roxybestgirl").join(&self.0)
    }
}
