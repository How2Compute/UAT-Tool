// This file contains the (windows-specific) handlers for getting engine-related information
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use winsafe::prelude::advapi_Hkey;

// Returns the path to the launcher data directory
#[cfg(target_family = "windows")]
pub fn get_launcher_data_dir() -> Result<PathBuf, &'static str> {
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

// Fetches the registered source builds from the registry
#[cfg(target_family = "windows")]
pub fn get_source_builds() -> Result<HashMap<String, PathBuf>, &'static str> {
    // Grab the Builds key from the registry, there will be one entry for each registered engine build
    let hkey = winsafe::HKEY::CURRENT_USER
        .RegOpenKeyEx(
            Some("SOFTWARE\\Epic Games\\Unreal Engine\\Builds"),
            winsafe::co::REG_OPTION::default(),
            winsafe::co::KEY::READ,
        )
        .or(Err("Unable to read source builds from registry"))?;

    // Iterate over the values in each key. Unfortunately lpData isn't returned, so we need to manually fetch the data for each key.
    // NOTE: The key name has a UUID for each engine version to identify it, and the data for that UUID is the path to the engine's root dir.
    let source_builds = hkey
        .RegEnumValue()
        .or(Err("Unable to enumerate registry value for source builds"))?
        .filter_map(|item| {
            if item.is_err() {
                return None;
            }

            let engine_key = item.unwrap().0;
            if let Some(engine_path_reg) = hkey.RegGetValue(None, Some(&engine_key)).ok() {
                match engine_path_reg {
                    winsafe::RegistryValue::Sz(s) => {
                        Some((engine_key, Path::new(&s).to_path_buf()))
                    }
                    _ => None, // We only support string paths
                }
            } else {
                None
            }
        })
        .collect();

    Ok(source_builds)
}
