use std::thread;

pub fn launch_steam_game(app_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("steam://run/{app_id}");
    thread::spawn(|| {
        // TODO: send result
        let _ = open::that(url);
    });
    Ok(())
}
