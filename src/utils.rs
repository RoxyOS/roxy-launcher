use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::error::RoxyResult;

pub fn launch_steam_game(
    app_id: u32,
    launch_result: LaunchResult,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("steam://run/{app_id}");
    thread::spawn(move || {
        *launch_result.lock().unwrap() = Some(open::that(url).map_err(Into::into));
    });
    Ok(())
}

pub type LaunchResult = Arc<Mutex<Option<RoxyResult>>>;
