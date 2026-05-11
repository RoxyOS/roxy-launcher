use std::{
    default,
    env::home_dir,
    fs,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

use egui::{Button, CentralPanel};
use egui_notify::Toasts;

use crate::{
    core::boot::on_booting,
    error::RoxyResult,
    profile::Profile,
    ui,
    utils::{LaunchResult, new_launch_result},
};

pub static DEFAULT_LAUNCHER_DIR: &'static str = "~/.local/share/roxy";

pub struct RoxyLauncher {
    pub message: Option<String>,
    pub toasts: Toasts,
    pub launch_result: LaunchResult,
    pub launching: bool,
    pub profiles: Vec<Profile>,
}

impl<'a> RoxyLauncher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let _ = cc;
        if !Profile::profile_root().exists() {
            fs::create_dir_all(Profile::profile_root());
        }

        Default::default()
    }

    fn finish_launch(&mut self, launch_result: RoxyResult) {
        self.launching = false;
        match launch_result {
            Ok(()) => self.toasts.success("Game launched"),
            Err(err) => self.toasts.error(err.to_string()),
        };
    }
}

impl<'a> eframe::App for RoxyLauncher {
    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        on_booting(self);
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("menubar").show_inside(ui, |ui| ui::toolbar::show(ui));
    }
}

impl<'a> Default for RoxyLauncher {
    fn default() -> Self {
        Self {
            message: None,
            toasts: Toasts::default(),
            launch_result: new_launch_result(),
            launching: false,
            profiles: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::RoxyError;

    use super::*;

    #[test]
    fn default_launcher_is_not_launching() {
        let launcher = RoxyLauncher::default();

        assert!(!launcher.launching);
    }

    #[test]
    fn finish_launch_clears_launching_after_success() {
        let mut launcher = RoxyLauncher {
            launching: true,
            ..Default::default()
        };

        launcher.finish_launch(Ok(()));

        assert!(!launcher.launching);
    }

    #[test]
    fn finish_launch_clears_launching_after_error() {
        let mut launcher = RoxyLauncher {
            launching: true,
            ..Default::default()
        };

        launcher.finish_launch(Err(RoxyError::GameNotInstalled));

        assert!(!launcher.launching);
    }
}
