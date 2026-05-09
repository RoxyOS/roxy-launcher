use std::{env::home_dir, fs, path::PathBuf};

use crate::{
    error::{RoxyError, RoxyResult},
    profile::Profile,
    utils::{LaunchResult, launch_steam_game},
};

const STS_GAME_ID: u32 = 2868840;

fn mods_dir() -> PathBuf {
    mods_dir_from_home(&home_dir().expect("HOME is not set"))
}

fn mods_dir_from_home(home: &std::path::Path) -> PathBuf {
    home.join(".local/share/Steam/steamapps/common/Slay the Spire 2")
        .join("mods")
}

fn copy_profile_contents(src: &std::path::Path, dest: &std::path::Path) -> RoxyResult {
    let options = fs_extra::dir::CopyOptions::new()
        .overwrite(true)
        .content_only(true);

    if dest.exists() {
        fs_extra::dir::remove(dest)?;
    }
    fs_extra::dir::copy(src, dest, &options)?;

    Ok(())
}

impl<'a> Profile<'a> {
    pub fn launch(&self, launch_result: LaunchResult) -> RoxyResult {
        self.ensure_valid_name()?;
        self.ensure_profile_exist()?;
        self.copy_profile_to_game()?;
        launch_raw(launch_result)?;
        Ok(())
    }

    fn ensure_profile_exist(&self) -> RoxyResult {
        ensure_profile_exists(&self.path())
    }

    fn copy_profile_to_game(&self) -> RoxyResult {
        let dest = mods_dir();
        let src = self.path();
        copy_profile_contents(&src, &dest)
    }
}

fn launch_raw(launch_result: LaunchResult) -> RoxyResult {
    launch_steam_game(STS_GAME_ID, launch_result).map_err(|_| RoxyError::GameNotInstalled)?;
    Ok(())
}

fn ensure_profile_exists(path: &std::path::Path) -> RoxyResult {
    if fs::exists(path)? {
        Ok(())
    } else {
        Err(RoxyError::ProfileDontExist)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use tempfile::tempdir;

    use super::*;
    use crate::utils::new_launch_result;

    #[test]
    fn mods_dir_is_nested_under_expected_game_path() {
        let fake_home = Path::new("/tmp/roxy-home");
        let expected = fake_home.join(".local/share/Steam/steamapps/common/Slay the Spire 2/mods");

        assert_eq!(mods_dir_from_home(fake_home), expected);
    }

    #[test]
    fn copy_profile_contents_copies_files_into_mods_root() {
        let tmp = tempdir().unwrap();
        let src = tmp.path().join("profile");
        let dest = tmp.path().join("mods");

        fs::create_dir_all(src.join("nested")).unwrap();
        fs::create_dir_all(&dest).unwrap();
        fs::write(src.join("mod.json"), "json").unwrap();
        fs::write(src.join("nested/data.txt"), "data").unwrap();

        copy_profile_contents(&src, &dest).unwrap();

        assert!(dest.join("mod.json").exists());
        assert!(dest.join("nested/data.txt").exists());
        assert!(!dest.join("profile").exists());
    }

    #[test]
    fn copy_profile_contents_removes_existing_mods_contents() {
        let tmp = tempdir().unwrap();
        let src = tmp.path().join("profile");
        let dest = tmp.path().join("mods");

        fs::create_dir_all(&src).unwrap();
        fs::create_dir_all(&dest).unwrap();
        fs::write(src.join("fresh.txt"), "fresh").unwrap();
        fs::write(dest.join("stale.txt"), "stale").unwrap();

        copy_profile_contents(&src, &dest).unwrap();

        assert!(dest.join("fresh.txt").exists());
        assert!(!dest.join("stale.txt").exists());
    }

    #[test]
    fn copy_profile_contents_creates_missing_destination() {
        let tmp = tempdir().unwrap();
        let src = tmp.path().join("profile");
        let dest = tmp.path().join("mods");

        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("fresh.txt"), "fresh").unwrap();

        copy_profile_contents(&src, &dest).unwrap();

        assert!(dest.join("fresh.txt").exists());
    }

    #[test]
    fn ensure_profile_exists_returns_ok_for_existing_directory() {
        let tmp = tempdir().unwrap();
        let profile = tmp.path().join("profile");
        fs::create_dir_all(&profile).unwrap();

        assert!(ensure_profile_exists(&profile).is_ok());
    }

    #[test]
    fn ensure_profile_exists_returns_profile_not_found_for_missing_directory() {
        let tmp = tempdir().unwrap();
        let missing = tmp.path().join("missing-profile");

        assert!(matches!(
            ensure_profile_exists(&missing),
            Err(RoxyError::ProfileDontExist)
        ));
    }

    #[test]
    fn launch_rejects_invalid_profile_name_without_touching_launch_result() {
        let profile = Profile::from("../escape".to_string());
        let launch_result = new_launch_result();

        assert!(matches!(
            profile.launch(launch_result.clone()),
            Err(RoxyError::InvalidProfileName)
        ));
        assert!(launch_result.lock().unwrap().is_none());
    }

    #[test]
    fn launch_rejects_missing_profile_without_touching_launch_result() {
        let tmp = tempdir().unwrap();
        let missing = tmp.path().join("missing-profile");
        let launch_result = new_launch_result();

        assert!(matches!(
            ensure_profile_exists(&missing),
            Err(RoxyError::ProfileDontExist)
        ));
        assert!(launch_result.lock().unwrap().is_none());
    }
}
