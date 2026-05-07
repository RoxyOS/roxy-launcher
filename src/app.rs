use egui::{Button, CentralPanel};
use egui_notify::Toasts;

use crate::{
    error::RoxyResult,
    profile::Profile,
    utils::{LaunchResult, new_launch_result},
};

pub struct RoxyLauncher {
    profile: String,
    message: Option<String>,
    toasts: Toasts,
    launch_result: LaunchResult,
    launching: bool,
}

impl RoxyLauncher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let _ = cc;
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

impl eframe::App for RoxyLauncher {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Profile: ");
            ui.text_edit_singleline(&mut self.profile);

            if let Some(message) = &self.message {
                ui.label(message);
            }

            let play_button = ui.add_enabled(!self.launching, Button::new("Play"));

            if play_button.clicked() {
                self.launching = true;
                match Profile(self.profile.clone()).launch(self.launch_result.clone()) {
                    Ok(()) => {
                        self.toasts.info("Starting game");
                    }
                    Err(err) => {
                        self.toasts.error(err.to_string());
                        self.launching = false;
                    }
                }
            }

            let launch_result = self.launch_result.lock().unwrap().take();
            if let Some(launch_result) = launch_result {
                self.finish_launch(launch_result);
            }
        });

        self.toasts.show(ui.ctx());
    }
}

impl Default for RoxyLauncher {
    fn default() -> Self {
        Self {
            profile: String::new(),
            message: None,
            toasts: Toasts::default(),
            launch_result: new_launch_result(),
            launching: false,
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
