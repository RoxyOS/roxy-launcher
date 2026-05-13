use egui::Ui;

use crate::{
    RoxyLauncher,
    profile::Profile,
    ui::dialogs::{RoxyAction, create_profile::CreateProfileDialogState},
};

impl RoxyLauncher {
    pub fn handle_create_profile_action(&mut self, ui: &mut Ui, action: RoxyAction) {
        match action {
            RoxyAction::CreateProfile => self
                .profiles
                .push(create_profile_by_dialog(&self.dialogs.create_profile)),
            _ => {}
        }
    }
}

fn create_profile_by_dialog(dialog: &CreateProfileDialogState) -> Profile {
    let result = Profile {
        name: dialog.name.clone(),
        data: Default::default(),
        enabled_mod: vec![],
        version_sts: "11.45.14".to_string(),
    };

    result.save_from_name();
    result
}
