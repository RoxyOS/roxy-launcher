use egui::{Color32, Frame, ScrollArea, Ui};

use crate::profile::Profile;
pub struct ProfilesGrid {
    pub selected_profile: Option<String>,
    pub opening_profile: Option<String>,
}

impl ProfilesGrid {
    pub fn show(&mut self, ui: &mut Ui, profiles: &Vec<Profile>) {}
}
