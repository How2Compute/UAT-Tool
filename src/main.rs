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

    let engine_installs = engine_utils::get_engine_installs().expect("Unable to fetch engine installs: ");
    for install in engine_installs {
        println!("- {:?}", install)
    }
}