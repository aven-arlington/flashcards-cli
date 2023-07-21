pub mod engine;

use std::env;
use std::process;
use crate::engine::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = engine::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

