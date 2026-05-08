use egui::Ui;

pub fn show(ui: &mut Ui) {
    let menu_bar = egui::MenuBar::new();
    menu_bar.ui(ui, |ui| {
        ui.menu_button("Instance", |ui| {
            if ui.button("Play").clicked() {
                todo!();
            }
            ui.separator();
            if ui.button("Edit").clicked() {
                todo!();
            }
            ui.separator();
            if ui.button("Dublicate").clicked() {
                todo!();
            }
            if ui.button("Delete").clicked() {
                todo!();
            }
        });
        ui.menu_button("Help", |ui| {
            if ui.button("Nothing").clicked() {
                todo!();
            }
        })
    });
}
