// Argument parsing logic to configure the application
use std::env;
use std::path::PathBuf;

pub struct Config {
    pub file_path: PathBuf,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            1 => {
                let file_path:PathBuf = Config::check_default_path().unwrap();
                Ok(Config {
                    file_path,
                })
            },
            2 => {
                let file_path:PathBuf = Config::check_path_argument(&args[1]).unwrap();
                Ok(Config {
                    file_path,
                })
            },
            _ => Err("Too many arguments"),
        }
    }

    fn check_default_path() -> Result<PathBuf, &'static str> {
        let mut path_buffer:PathBuf = env::current_exe().unwrap();
        path_buffer.pop();
        path_buffer.push("flashcards.json");
        let path_str = path_buffer.as_path().display().to_string();
        println!("Checking for default configuration file: {}", path_str);
        if path_buffer.try_exists().unwrap() {
            Ok(path_buffer)
        } else {
            Err("The flashcards.json configuration file could not be found")
        }
    }

    fn check_path_argument(arg_path: &String) -> Result<PathBuf, &'static str> {
        let path_buffer:PathBuf = PathBuf::from(arg_path);
        let path_str = path_buffer.as_path().display().to_string();
        println!("Using custom configuration file path: {}", path_str);
        if path_buffer.try_exists().unwrap() {
            Ok(path_buffer)
        } else {
            Err("The file provided in the argument does not exist")
        }
    }
}


