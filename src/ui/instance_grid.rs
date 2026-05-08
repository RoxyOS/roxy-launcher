use egui::{Color32, Frame, ScrollArea};

use crate::profile::Profile;

pub fn show(ui: &mut egui::Ui, profiles: &[Profile], selected: Option<Profile>) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            for profile in profiles {
                let profile_name = &profile.name;
                let is_selected = if let Some(select) = &selected {
                    &select.name == profile_name
                } else {
                    false
                };

                let frame = Frame::default()
                    .fill(if is_selected {
                        Color32::from_rgb(60, 80, 120)
                    } else {
                        Color32::from_rgb(35, 35, 50)
                    })
                    .corner_radius(8.0)
                    .inner_margin(8.0);
                frame.show(ui, |ui| {
                    ui.set_width(80.0);
                    ui.label(" ");
                    ui.label(profile_name);
                });
            }
        })
    });
}
