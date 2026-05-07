use egui::{Button, CentralPanel};
use egui_notify::Toasts;

use crate::{profile::Profile, utils::LaunchResult};

#[derive(Default)]
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

            if let Some(launch_result) = self.launch_result.lock().unwrap().take() {
                self.launching = false;
                match launch_result {
                    Ok(()) => self.toasts.success("Game launched"),
                    Err(err) => self.toasts.error(err.to_string()),
                };
            }
        });

        self.toasts.show(ui.ctx());
    }
}
