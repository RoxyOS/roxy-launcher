use egui::Ui;

use crate::{RoxyLauncher, ui::dialogs::RoxyAction};

pub mod dialog;
pub mod toolbar;

impl RoxyLauncher {
    pub fn handle_action(&mut self, action: Option<RoxyAction>, ui: &mut Ui) {
        if action.is_none() {
            return;
        }
        let action = action.unwrap();
        match action {
            RoxyAction::CreateProfile | RoxyAction::CancelCreateProfile => {
                self.handle_create_profile_action(ui, action)
            }
            _ => {}
        }
    }
}
