use std::{fmt::Display, io};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RoxyError {
    InvalidProfileName,
    ProfileDontExist,
    GameNotInstalled,
    IOError(String),
    ProfileParseError(String),
}

pub type RoxyResult<T = ()> = Result<T, RoxyError>;

impl From<fs_extra::error::Error> for RoxyError {
    fn from(error: fs_extra::error::Error) -> Self {
        Self::IOError(error.to_string())
    }
}

impl From<io::Error> for RoxyError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}

impl Display for RoxyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProfileName => f.write_str("Profile name is invalid"),
            Self::ProfileDontExist => f.write_str("Profile does not exist"),
            Self::GameNotInstalled => f.write_str("Steam or Slay the Spire 2 is not installed"),
            Self::IOError(message) => write!(f, "{message}"),
            Self::ProfileParseError(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for RoxyError {}
