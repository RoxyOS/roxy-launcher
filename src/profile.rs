use std::path::PathBuf;

use tap::Tap;

use crate::LAUNCHER_DATA;

pub struct Profile(pub String);

impl From<String> for Profile {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Profile {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(LAUNCHER_DATA).join(self.0.clone())
    }
}
