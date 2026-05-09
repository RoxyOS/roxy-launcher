pub fn show(ui: &mut egui::Ui, selected: &Option<String>) {
    ui.set_min_width(160.0);
    if let Some(name) = selected {
        ui.heading(name);
        ui.separator();
        ui.add_space(8.0);

        for label in ["Launch", "Edit", "Duplicate", "Delete"] {
            if ui
                .add(egui::Button::new(label).min_size(egui::vec2(140.8, 28.0)))
                .clicked()
            {}
            ui.add_space(4.0);
        }
    } else {
        ui.label("Nothing here...");
    }
}
