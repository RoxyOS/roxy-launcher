use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct StsMod {
    pub name: String,
    pub version: ModVersion,
    pub icon_uri: Option<String>,
    pub desc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
