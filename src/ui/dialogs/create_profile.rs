use std::path::PathBuf;

use egui::Ui;
use url::Url;

use crate::{ui::dialogs::RoxyAction, utils::path_util::expand_home_dir_string};

#[derive(Debug)]
pub struct CreateProfileDialogState {
    pub name: String,
    pub mods_dir: String,
    pub image_path: String,
    pub open: bool,
}

pub static CREATE_PROFILE_WINDOW_ID: &str = "CreateProfile";
impl Default for CreateProfileDialogState {
    fn default() -> Self {
        Self {
            name: String::new(),
            mods_dir: String::new(),
            image_path: String::new(),
            open: false,
        }
    }
}

impl CreateProfileDialogState {
    pub fn show(&mut self, ui: &mut Ui) -> Option<RoxyAction> {
        if !self.open {
            return None;
        }

        let mut result_action = None;
        egui::Window::new(CREATE_PROFILE_WINDOW_ID)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                ui.vertical(|ui| {
                    ui.label("ProfileNam");
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.name).hint_text("silent modded..."),
                    );
                    ui.add_space(3.0);

                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("icon path");
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.image_path)
                                    .hint_text("/path/to/your/image"),
                            );
                        });
                        let absolute_image_path = expand_home_dir_string(&self.image_path);
                        ui.add(egui::Image::new(
                            match Url::from_file_path(absolute_image_path) {
                                Ok(ok) => ok.to_string(),
                                Err(_) => String::new(),
                            },
                        ));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("create").clicked() {
                            result_action = Some(RoxyAction::CreateProfile);
                            self.open = false;
                        }
                        if ui.button("cancle").clicked() {
                            result_action = Some(RoxyAction::CancelCreateProfile);
                            self.open = false;
                        }
                    })
                });
            });
        result_action
    }
}
