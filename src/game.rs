use crate::{profile::Profile, utils::launch_steam_game};

const STS_GAME_ID: u32 = 2868840;

pub fn launch(profile: Profile) {
    copy_profile_to_game(profile);
    launch_raw();
}

fn copy_profile_to_game(profile: Profile) {
    todo!()
}

fn launch_raw() {
    launch_steam_game(STS_GAME_ID).unwrap()
}
