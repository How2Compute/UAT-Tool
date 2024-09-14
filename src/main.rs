use clap::Parser;
mod engine_utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Version of Unreal Engine to use
    engine_version: String,

    /// The UAT command to run
    #[arg(trailing_var_arg = true, required = true)]
    command: Vec<String>
}

fn main() {
    let args = Args::parse();
    let launcher_builds = engine_utils::get_launcher_builds().expect("Unable to get launcher builds");
    let source_builds = engine_utils::get_source_builds().expect("Unable to get source builds");
    println!("Launcher builds {:?}!", launcher_builds);
    println!("Source builds {:?}!", source_builds);
    println!("ARGS: {:?}", args);

    launcher_builds.iter().for_each(|(key, value)| {
        engine_utils::get_engine_version(value).expect("EVR: ");
    });
    source_builds.iter().for_each(|(key, value)| {
        engine_utils::get_engine_version(value).expect("EVR: ");
    });
    
}