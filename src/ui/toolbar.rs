use egui::Ui;

use crate::ui::dialogs::RoxyAction;

#[derive(Default, Debug)]
pub struct ToolBar {}

pub enum ToolBarAction {
    Create,
    Edit,
    Delete,
    Play,
    Help,
    Duplicate,
}

impl ToolBar {
    pub fn show(&mut self, ui: &mut Ui) -> Option<RoxyAction> {
        let menu_bar = egui::MenuBar::new();
        let mut result = None;
        menu_bar.ui(ui, |ui| {
            ui.menu_button("Instance", |ui| {
                if ui.button("Play").clicked() {
                    result = Some(RoxyAction::ToolBarPlay);
                }
                ui.separator();
                if ui.button("Create").clicked() {
                    result = Some(RoxyAction::ToolBarCreate);
                }
                if ui.button("Edit").clicked() {
                    result = Some(RoxyAction::ToolBarEdit);
                }
                ui.separator();
                if ui.button("Duplicate").clicked() {
                    result = Some(RoxyAction::ToolBarDuplicate);
                }
                if ui.button("Delete").clicked() {
                    result = Some(RoxyAction::ToolBarDelete);
                }
            });
            ui.menu_button("Help", |ui| {
                if ui.button("Nothing").clicked() {
                    result = Some(RoxyAction::ToolBarHelp);
                }
            })
        });
        result
    }
}
