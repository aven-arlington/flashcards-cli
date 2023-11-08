pub mod engine;
use crate::engine::config::Config;
use std::io::Write;
use log::error;
use std::process;

// Type the following in powershell to create the debug environment variable
// $env:RUST_LOG = "debug"
// Replace "debug" with desired logging level: info, warn, debug, error 
fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {}:{} - {}",
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    let config = Config::build().unwrap_or_else(|err| {
        error!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = engine::run(config) {
        error!("Application error: {e}");
        process::exit(1);
    }
}

