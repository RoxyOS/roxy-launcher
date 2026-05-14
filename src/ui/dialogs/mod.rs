use egui::Ui;

use crate::{error::RoxyResult, ui::dialogs::create_profile::CreateProfileDialogState};

pub mod create_profile;

#[derive(Default, Debug)]
pub struct DialogsStates {
    pub create_profile: CreateProfileDialogState,
}

#[derive(Debug)]
pub enum RoxyAction {
    CreateProfile,
    CancelCreateProfile,
    ToolBarCreate,
    ToolBarHelp,
    ToolBarDuplicate,
    ToolBarEdit,
    ToolBarPlay,
    ToolBarDelete,
}

impl DialogsStates {
    pub fn poll(&mut self, ui: &mut Ui) -> Option<RoxyAction> {
        if self.create_profile.open {
            return self.create_profile.show(ui);
        }
        None
    }
}
