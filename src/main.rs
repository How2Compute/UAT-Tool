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

    println!("Launcher builds {:?}!", launcher_builds);
    println!("ARGS: {:?}", args);
    
}