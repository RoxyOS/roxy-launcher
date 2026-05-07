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

pub fn new_launch_result() -> LaunchResult {
    Arc::new(Mutex::new(None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_launch_result_starts_empty() {
        let launch_result = new_launch_result();

        assert!(launch_result.lock().unwrap().is_none());
    }
}
