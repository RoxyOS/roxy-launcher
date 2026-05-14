#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod core;
mod error;
mod game;
pub mod language;
mod profile;
pub mod ui;
pub mod utils;

pub use app::RoxyLauncher;
