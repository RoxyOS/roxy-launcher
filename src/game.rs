use std::{fs, path::PathBuf};

use crate::{
    error::{RoxyError, RoxyResult},
    profile::Profile,
    utils::launch_steam_game,
};

const STS_GAME_ID: u32 = 2868840;
const GAME_DIR: &str = "~/.local/share/Steam/steamapps/common/Slay the Spire 2/";

impl Profile {
    pub fn launch(&self) -> RoxyResult {
        self.ensure_profile_exist()?;
        self.copy_profile_to_game()?;
        launch_raw()?;
        Ok(())
    }

    fn ensure_profile_exist(&self) -> RoxyResult {
        if fs::exists(self.path())? {
            Ok(())
        } else {
            Err(RoxyError::ProfileDontExist)
        }
    }

    fn copy_profile_to_game(&self) -> RoxyResult {
        let dest = PathBuf::from(GAME_DIR).join("mods");
        let src = self.path();
        let options = fs_extra::dir::CopyOptions::new().overwrite(true);

        fs_extra::dir::copy(src, dest, &options)?;

        Ok(())
    }
}

fn launch_raw() -> RoxyResult {
    launch_steam_game(STS_GAME_ID).unwrap();
    Ok(())
}
