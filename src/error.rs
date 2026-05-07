use core::error;
use std::{fmt::Display, fs, io};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RoxyError {
    ProfileDontExist,
    IOError(String),
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
            Self::ProfileDontExist => f.write_str("Profile does not exist"),
            Self::IOError(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for RoxyError {}
