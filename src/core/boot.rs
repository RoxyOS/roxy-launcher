use std::{fs, path::Path};

use egui::Context;

use crate::{RoxyLauncher, app, utils::path_util};

pub fn on_booting(app: &mut RoxyLauncher) {
    let path_data_dir_string =
        path_util::expand_home_dir_string(app::DEFAULT_LAUNCHER_DIR.to_string());
    let path_data_dir = Path::new(&path_data_dir_string);
    if !path_data_dir.exists()
        && let Err(e) = fs::create_dir(path_data_dir)
    {
        app.toasts.error(format!(
            "Cant use the data directory at {}! \n because of {}",
            app::DEFAULT_LAUNCHER_DIR,
            e
        ));
    }
}
