#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod game;
mod profile;
mod utils;
pub use app::RoxyLauncher;

#[cfg(target_os = "linux")]
pub const LAUNCHER_DATA: &str = "~/.roxybestgirl";
#[cfg(not(target_os = "linux"))]
pub const LAUNCHER_DATA: &str = "TODO";
