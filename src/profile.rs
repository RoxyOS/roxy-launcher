use std::path::PathBuf;

use tap::Tap;

use crate::LAUNCHER_DATA;

pub struct Profile {
    pub name: String,
}

impl Profile {
    fn path(&self) -> PathBuf {
        PathBuf::from(LAUNCHER_DATA).join(self.name.clone())
    }
}
