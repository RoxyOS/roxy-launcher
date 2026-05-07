use crate::{error::RoxyError, profile::Profile};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RoxyLauncher {
    profile: String,
    message: Option<String>,
}

impl RoxyLauncher {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for RoxyLauncher {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label("Profile: ");
        ui.text_edit_singleline(&mut self.profile);

        if let Some(message) = &self.message {
            ui.label(message);
        }

        ui.menu_button("Play", |ui| {
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
