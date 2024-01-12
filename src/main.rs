use std::process;

fn main() {
    let config = flashcards_cli::Config::build().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = flashcards_cli::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

