use egui::CentralPanel;

use crate::{error::RoxyError, profile::Profile};

#[derive(Default, Debug)]
pub struct RoxyLauncher {
    profile: String,
    message: Option<String>,
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

        if ui.button("Play").clicked() {
            match Profile(self.profile.clone()).launch() {
                    Ok(()) => {
                        self.message = Some("Game started. this might take a long time due to the launcher is starting sts2 via steam.".into())
                    }
                    Err(err) => self.message = Some(err.to_string()),
                }
        }
        });
    }
}
