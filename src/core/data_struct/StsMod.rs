#[derive(Debug)]
pub struct StsMod {
    pub name: String,
    pub version: ModVersion,
    pub icon_uri: Option<String>,
    pub desc: String,
}

#[derive(Debug)]
pub struct ModVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
