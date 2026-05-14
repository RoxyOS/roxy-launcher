use std::sync::{LazyLock, RwLock};

pub static LANGUAGE_ELEMENT: LazyLock<Vec<&str>> = LazyLock::new(|| {
    let mut v = vec![];

    //toobar
    v.append(&mut vec![
        "toolbar.settings",
        "toolbar.help",
        "toolbar.profile.create",
        "toolbar.profile.delete",
        "toolbar.profile.play",
        "toolbar.profile.duplicate",
        "toolbar.profile.edit",
    ]);

    v
});
