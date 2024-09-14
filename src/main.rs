use std::process::Command;

use clap::Parser;
use colored::Colorize;
use engine_utils::EngineInstall;
mod engine_utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long = "list", short = 'o')]
    should_list: bool,

    /// Version(s) of Unreal Engine to use - "+" as separator
    engine_version: String,

    /// The UAT command to run
    #[arg(trailing_var_arg = true, required = true)]
    command: Vec<String>
}

// Helper function for matching a string to an engine number
// This supports different "levels" (e.g. adding/omitting patch version) + source builds
// Supported inputs:
// A version number, e.g. "5.4.2" OR "5.4" OR "5"
// "source-" (w/ engine number): "source-X[.X][.X]"
// A CL number: "cl-XXX"
// "latest": will grab the highest version number (looking at major, then minor, then patch; ignores CL)
fn find_engine_version(installs: &Vec<EngineInstall>, name: &str) -> Option<EngineInstall> {
    if let Some(cl_version_str) = name.strip_prefix("cl-") {
        if let Some(cl_version_id) = cl_version_str.parse::<u32>().ok() {
            let res = installs.iter().find(|item| item.cl_version == cl_version_id);
            
            // Return a copy of the inner value
            return match res {
                Some(install) => Some(install.clone()),
                None => None
            };
        }

        // Unable to find a version matching the CL
        return None;
    }
    else if (name == "latest") {
        // NOTE: to simplify sorting by version, we can turn major, minor, version into a number and sort that from smallest -> largest (sort_by_key default)
        // NOTE: Assumes that a version number will be no more than 2 digits (which, so far, has worked for UE versions)
        // NOTE: Clone + sort makes things easier, and the vector shouldn't be too large; TLDR: don't care about the performance hit
        let mut mut_installs = installs.clone();
        mut_installs.sort_by_key(|item| item.major_version as u32 * 10000 + item.minor_version as u32 * 100 + item.patch_version as u32);
        
        // Deref the last elem (this has the highest computed version number -> should be the latest install)
        match mut_installs.last() {
            Some(install_ref) => Some(install_ref.clone()),
            None => None
        }
    }
    else {
        // [source]-VERSION
        // Create something both of them can use
        let (version_name, needs_source_build) = match name.strip_prefix("source-") {
            Some(version) => (version, true),
            None => (name, false)
        };

        // Parse the engine version components (and ensure it looks valid)
        let major_version: Option<u8> = version_name.split('.').nth(0).or(Some("")).unwrap().parse().ok();
        let minor_version: Option<u8> = version_name.split('.').nth(1).or(Some("")).unwrap().parse().ok();
        let patch_version: Option<u8> = version_name.split('.').nth(2).or(Some("")).unwrap().parse().ok();

        if (patch_version.is_some() && minor_version.is_none() || major_version.is_none()) {
            println!("{}: {}", "INVALID ENGINE VERSION".bold().red(), name);
            return None;
        }
        
        // 1st filter -> we either don't need a source build, or we do need one but the current one is a source build 
        let res = installs.iter().filter(|item| !needs_source_build || item.is_source).find(|item|
            major_version.unwrap_or(item.major_version) == item.major_version &&
            minor_version.unwrap_or(item.minor_version) == item.minor_version && 
            patch_version.unwrap_or(item.patch_version) == item.patch_version
        );

        // Clone the inner value
        match res {
            Some(found) => Some(found.clone()),
            None => None
        }

    }
    
}

fn main() {
    // Parse the CLI and pull the versions of UE that are currently installed
    let args = Args::parse();
    let engine_installs = engine_utils::get_engine_installs().expect("Unable to fetch engine installs: ");

    // Were we run with --list?
    if args.should_list {
        // List subcommand used - only list the installed engine versions
        println!("{}", "Unreal Engine Installs".bold().underline());
        for install in engine_installs {
            let version_string = format!("{}.{}.{} {}", install.major_version, install.minor_version, install.patch_version, match install.is_source {
                true => "[SOURCE]".bold(),
                false => "[LAUNCHER]".bold()  // .bold() gives us a conversion, but won't do anything
            });
            
            println!("- {}\t@\t {}", version_string, match install.base_dir.to_str() {
                Some(path) => path.italic(),
                None => "UNKNOWN".red().bold()
            })
        }
        return;
    }
    // Regular run
    else {
        // Split & look up the passed in Unreal Engine versions
        let requested_engine_names: Vec<&str> = args.engine_version.split("+").collect();
        let engines_to_use: Vec<(&str, Option<EngineInstall>)> = requested_engine_names.iter().map(|&version_name| (version_name, find_engine_version(&engine_installs, version_name))).collect();
        let engines_not_found: Vec<&(&str, Option<EngineInstall>)> = engines_to_use.iter().filter(|(_, engine)| engine.is_none()).collect();

        // Ensure all version names were matched to an engine on disk
        if !engines_not_found.is_empty() {
            // At least one engine version failed lookup!
            println!("{}:", "Unable to resolve the following engine version(s)".bold().red());
            for (name, _) in engines_not_found {
                println!("- {}", name);
            };

            return;
        }
        
        // Log the versions found to the user
        println!("{}:", "Found engine version(s) at".bold().green());
        for (name, engine) in &engines_to_use {
            println!("- {} -> {}", name, engine.clone().unwrap().base_dir.to_str().unwrap_or("PATH ERROR"));
        }
        if engines_to_use.len() > 1 {
            println!("{} {}", engines_to_use.len().to_string().bold().yellow(), "versions passed in!".bold().green());
            println!("{}", "We will run your command once for each version, in the order listed above".italic());
        }

        for (name, engine) in engines_to_use {
            println!("{} {}", "Running for version: ".bold(), name.yellow().bold());
            static UAT_RELATIVE_PATH: &str = "Engine/Build/BatchFiles/RunUAT.bat";
            let uat_path = engine.unwrap().base_dir.join(UAT_RELATIVE_PATH);
            
            // Log the command we are going to run
            // TODO: Fix this weird iter clone thingy! There's got to be a better way!
            let user_command = args.command.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" ");
            let os_command = format!("{:?} {}", uat_path, user_command);
            println!("Running: {}", os_command.italic());

            // Start UAT with the command, wait for it to finish and log the code
            let mut uat_invocation = Command::new(uat_path.as_os_str())
            .args(args.command.iter())
            .spawn()
            .expect("UAT command failed to start!");
            let exit_code = uat_invocation.wait().expect("Unable to wait for UAT completion").code().expect("Unable to retrieve output code");
            println!("{}: {}", "UAT process exited with code".bold(), exit_code.to_string().yellow().bold());

            // Fail the entire process if one exit code is non-zero/a failure
            // TODO: Turn this into something optional?
            if (exit_code != 0) {
                println!("{}", "Non-zero exit code! Assuming process has failed, stopping further execution!".red().bold());
                return;
            }
        }

        println!("{}", "DONE!".bold().green())
    }
}