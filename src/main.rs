use flashcards_cli::Config;

fn main() {
    let config = Config::build();

    flashcards_cli::run(config).unwrap();
}

