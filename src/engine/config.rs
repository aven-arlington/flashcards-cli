use crate::engine::flashcard::FlashCard;
use log::debug;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub attempts_before_hint: usize,
    pub attempts_before_wrong: usize,
    pub cards: Vec<FlashCard>,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let file_path = Config::check_default_path().unwrap();

        debug!("Parsing yaml file: {}", file_path.as_path().display());
        let file_data = fs::read_to_string(file_path).expect("Unable to read config file");
        debug!("File data:\n{}", file_data);

        let conf: Config =
            serde_yaml::from_str(file_data.as_str()).expect("Unable to parse data string");

        assert!(
            conf.attempts_before_hint != 0,
            "Invalid attmpts_before_hint"
        );
        assert!(
            conf.attempts_before_wrong != 0,
            "Invalid attempts_before_wrong"
        );
        assert!(!conf.cards.is_empty(), "No FlashCards in config file");
        Ok(conf)
    }

    fn check_default_path() -> Result<PathBuf, &'static str> {
        let mut path_buffer: PathBuf = env::current_dir().unwrap();
        path_buffer.push("flashcards.yaml");
        debug!(
            "Checking for default configuration file: {}",
            path_buffer.as_path().display().to_string()
        );
        if path_buffer.try_exists().unwrap() {
            Ok(path_buffer)
        } else {
            Err("The flashcards.yaml configuration file could not be found")
        }
    }
}

