use std::{
    env::home_dir,
    path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR, Path},
};

use egui::TextBuffer;

use crate::utils::string_util;

pub fn expand_home_dir_string(mut path: &str) -> String {
    if let Some(s) = path.strip_prefix("~") {
        if let Some(home_dir) = home_dir() {
            return format!("{}{}", home_dir.to_string_lossy(), s);
        } else {
            return format!("/{}", s);
        }
    }
    path.into()
}

pub fn parent_or_cur_dir(mut path: String, out_filename: &mut String) -> String {
    let p = Path::new(&path);
    *out_filename = p
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    p.parent()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".to_string())
}

pub fn name_without_ext(name: &str) -> &str {
    Path::new(name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(name)
}
