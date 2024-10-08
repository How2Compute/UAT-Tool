use crate::platform_utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// Based on an entry in InstallationList of LauncherInstalled.dat
// NOTE: We need the non-snake case because that's how the file stores things
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct LauncherInstallEntry {
    InstallLocation: String,
    NamespaceId: String,
    ItemId: String,
    ArtifactId: String,
    AppVersion: String,
    AppName: String,
}

// Based on LauncherInstalled.dat
// NOTE: We need the non-snake case because that's how the file stores things
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct LauncherInstalledDat {
    InstallationList: Vec<LauncherInstallEntry>,
}

// Based on LauncherInstalled.dat
// NOTE: We need the non-snake case because that's how the file stores things
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct BuildVersionFile {
    MajorVersion: u8,
    MinorVersion: u8,
    PatchVersion: u8,
    Changelist: u32,
    CompatibleChangelist: u32, // Included mainly in source builds, "cl-" will match to this is Changelist=0
    IsLicenseeVersion: u8,     // NOTE: Seems to be a flag/boolean val, but stored as a 0(/1?)
    IsPromotedBuild: u8,       // NOTE: Seems to be a flag/boolean val, but stored as a 0(/1?)
    BranchName: String,
}

// This struct contains a representation of an Unreal Engine install.
// NOTE: Every version should have a major/minor/patch version associated with it! Pretty much: we require a valid Build.version
#[derive(Debug, Clone)]
pub struct EngineInstall {
    // Unreal Engine version: {MAJOR}.{MINOR}.{PATCH}, e.g. 5.4.3 -> MAJOR=5, MINOR=4, PATCH=3
    pub major_version: u8,
    pub minor_version: u8,
    pub patch_version: u8,
    pub cl_version: u32,

    // Whether or not this is a source build of the engine
    pub is_source: bool,

    // Base directory of the Unreal Engine installatino
    pub base_dir: PathBuf,
}

/*
 * This file contains all of the code we need to figure out where the Unreal Engine installations live.
 * This function returns a HashMap with the engine version (with ENGINE_PREFIX removed) as key and the "base directory" as the value
 */
pub fn get_launcher_builds() -> Result<HashMap<String, PathBuf>, &'static str> {
    // Attempt to read the LauncherInstalled.dat file, which contains info about everything installed by the launcher
    // NOTE: This is more than just the engines - it also includes things like plugins
    let launcher_installed_path =
        platform_utils::get_launcher_data_dir()?.join("LauncherInstalled.dat");
    let install_dat =
        fs::read_to_string(launcher_installed_path).or(Err("Unable to read launcher DAT"))?;

    // Parse the LauncherInstaled.dat file
    // Identify the Unreal Engine installations - this is done by checking if AppName starts with "UE_" per e.g.
    // https://github.com/EpicGames/UnrealEngine/blob/40eea367040d50aadd9f030ed5909fc890c159c2/Engine/Extras/P4VUtils/Commands/SubmitAndVirtualizeCommand.cs#L907
    static ENGINE_PREFIX: &str = "UE_";
    let deserialized: LauncherInstalledDat =
        serde_json::from_str(&install_dat).or(Err("Unable to parse launcher DAT"))?;
    Ok(deserialized
        .InstallationList
        .into_iter()
        .filter_map(|entry| match entry.AppName.strip_prefix(ENGINE_PREFIX) {
            Some(version_num) => Some((
                version_num.to_string(),
                Path::new(&entry.InstallLocation).to_path_buf(),
            )),
            None => None,
        })
        .collect())
}

pub fn get_engine_installs() -> Result<Vec<EngineInstall>, &'static str> {
    let mut engine_installs: Vec<EngineInstall> = get_launcher_builds()?
        .into_iter()
        .filter_map(|(name, path)| to_engine_install(&name, false, path))
        .collect();
    let source_installs: Vec<EngineInstall> = platform_utils::get_source_builds()?
        .into_iter()
        .filter_map(|(name, path)| to_engine_install(&name, true, path))
        .collect();
    engine_installs.extend(source_installs);

    return Ok(engine_installs);
}

// Helper function used to turn a launcher/source build into an EngineInstall by pulling their Build.version/etc.
pub fn to_engine_install(_name: &str, is_source: bool, path: PathBuf) -> Option<EngineInstall> {
    if let Some(version_info) = get_engine_version(&path).ok() {
        return Some(EngineInstall {
            base_dir: path,
            cl_version: match version_info.Changelist {
                0 => version_info.CompatibleChangelist,
                i => i,
            },
            major_version: version_info.MajorVersion,
            minor_version: version_info.MinorVersion,
            patch_version: version_info.PatchVersion,
            is_source: is_source,
        });
    }

    return None;
}

// Helper function that reads the Build.version file which has info about the major/minor/etc. version
// NOTE: This may not be too useful for custom branches of the editor, but should at least allow
//       us to capture launcher builds + "regular" git clones
fn get_engine_version(engine_base_dir: &PathBuf) -> Result<BuildVersionFile, &'static str> {
    // Read + parse the Build.version file. This should be present for all launcher/source builds and gives us a "standardized"
    // way of reading engine versions.
    let build_version_path = engine_base_dir.join("Engine/Build/Build.version");
    let build_version_file =
        fs::read_to_string(build_version_path).or(Err("Unable to read Build.version file"))?;
    let deserialized: BuildVersionFile =
        serde_json::from_str(&build_version_file).or(Err("Unable to parse Build.version file"))?;

    return Ok(deserialized);
}
