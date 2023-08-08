pub mod engine;
use std::env;
use std::process;
use crate::engine::config::Config;
use log::error;

// Type the following in powershell to create the debug environment variable
// $env:RUST_LOG = "debug"
// Replace "debug" with desired logging level: info, warn, debug, error 
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        error!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = engine::run(config) {
        error!("Application error: {e}");
        process::exit(1);
    }
}

