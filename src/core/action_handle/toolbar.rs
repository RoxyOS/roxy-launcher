use egui::Ui;

use crate::{
    RoxyLauncher,
    ui::{dialogs::RoxyAction, toolbar::ToolBarAction},
};

impl RoxyLauncher {
    pub fn handle_toolbar_action(&mut self, ui: &mut Ui, action: Option<RoxyAction>) {
        if action.is_none() {
            return;
        }
        match action.unwrap() {
            RoxyAction::ToolBarCreate => {
                self.dialogs.create_profile.open = true;
            }
            _ => {}
        }
    }
}
