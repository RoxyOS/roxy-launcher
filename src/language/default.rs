use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

pub static DEFAULT_EN_LANG_MAP: LazyLock<RwLock<HashMap<&str, String>>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    //toolbar
    m.insert("toolbar.settings", "setting".to_string());
    m.insert("toolbar.help", "Help".to_string());
    m.insert("toolbar.profile.create", "Create".to_string());

    RwLock::new(m)
});
