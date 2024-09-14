use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// TODO: platform-conditional compile (this is win-specific code)
fn get_launcher_data_dir() -> Result<PathBuf, &'static str> {
    let res = winsafe::SHGetKnownFolderPath(
        &winsafe::co::KNOWNFOLDERID::ProgramData,
        winsafe::co::KF::DEFAULT,
        None,
    );

    match res {
        Ok(path) => Ok(Path::new(&path)
            .join("Epic/UnrealEngineLauncher")
            .to_path_buf()),
        Err(_) => Err("Unable to get launcher path"),
    }
}

// Based on an entry in InstallationList of LauncherInstalled.dat
#[derive(Debug, Serialize, Deserialize)]
struct LauncherInstallEntry {
    InstallLocation: String,
    NamespaceId: String,
    ItemId: String,
    ArtifactId: String,
    AppVersion: String,
    AppName: String,
}

// Based on LauncherInstalled.dat
#[derive(Debug, Serialize, Deserialize)]
struct LauncherInstalledDat {
    InstallationList: Vec<LauncherInstallEntry>,
}

/*
 * This file contains all of the code we need to figure out where the Unreal Engine installations live.
 * This function returns a HashMap with the engine version (with ENGINE_PREFIX removed) as key and the "base directory" as the value
 */
pub fn get_launcher_builds() -> Result<HashMap<String, PathBuf>, &'static str> {
    // Attempt to read the LauncherInstalled.dat file, which contains info about everything installed by the launcher
    // NOTE: This is more than just the engines - it also includes things like plugins
    let launcher_installed_path = get_launcher_data_dir()?.join("LauncherInstalled.dat");
    let install_dat = fs::read_to_string(launcher_installed_path).or(Err("Unable to read launcher DAT"))?;

    // Parse the LauncherInstaled.dat file
    // Identify the Unreal Engine installations - this is done by checking if AppName starts with "UE_" per e.g.
    // https://github.com/EpicGames/UnrealEngine/blob/40eea367040d50aadd9f030ed5909fc890c159c2/Engine/Extras/P4VUtils/Commands/SubmitAndVirtualizeCommand.cs#L907
    static ENGINE_PREFIX: &str = "UE_";
    let deserialized: LauncherInstalledDat = serde_json::from_str(&install_dat).or(Err("Unable to parse launcher DAT"))?;
    Ok(deserialized
        .InstallationList
        .into_iter()
        .filter_map(|entry| match entry.AppName.strip_prefix(ENGINE_PREFIX) {
                Some(version_num) => Some((version_num.to_string(), Path::new(&entry.InstallLocation).to_path_buf())),
                None => None
        })
        .collect())
}
