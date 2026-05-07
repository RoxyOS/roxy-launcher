pub fn launch_steam_game(app_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("steam://run/{app_id}");
    open::that(url)?;
    Ok(())
}
