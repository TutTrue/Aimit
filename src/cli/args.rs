use clap::{Arg, Command};
use dialoguer::Select;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command as SysCommand;

use crate::config::settings;
use crate::models::ModelType;

pub struct Cli;

impl Cli {
    pub fn run() -> ModelType {
        let matches: clap::ArgMatches = Command::new("aimit")
            .version("1.0")
            .author("Mahmoud Hamdy (TutTrue) <mahmoud.hamdy5113@gmail.com>")
            .about("Generates commit messages using AI")
            .arg(
                Arg::new("prompt")
                    .short('p')
                    .long("prompt")
                    .help("Sets the prompt for the AI model")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with_all(["default", "model", "key"]),
            )
            .arg(
                Arg::new("default")
                    .short('d')
                    .long("default")
                    .help("Sets the default AI model")
                    .num_args(0..=1)
                    .action(clap::ArgAction::Set)
                    .value_name("MODEL")
                    .conflicts_with_all(["model", "key"]),
            )
            .arg(
                Arg::new("model")
                    .short('m')
                    .long("model")
                    .num_args(0..=1)
                    .action(clap::ArgAction::Set)
                    .help("Select the AI model to use"),
            )
            .arg(
                Arg::new("key")
                    .short('k')
                    .long("key")
                    .help("Edit the API key for the selected model"),
            )
            .get_matches();
        let settings = settings::Settings::new().unwrap();
        let mut model = settings.get_default_model().clone();

        if matches.get_flag("prompt") {
            Self::update_prompt().unwrap();
            std::process::exit(0);
        } else if matches.contains_id("default") {
            let default = matches.get_one::<String>("default");
            Self::update_default_model(default.cloned());
        } else if matches.contains_id("key") {
            Self::set_api_key(matches);
        } else if matches.contains_id("model") {
            let selected_model = matches.get_one::<String>("model");
            model = Self::select_current_model(selected_model.cloned());
        }
        model
    }

    fn select_model() -> io::Result<ModelType> {
        let models = vec!["Gemini"];
        let selection = Select::new()
            .with_prompt("Select the AI model to use")
            .items(&models)
            .default(0)
            .interact()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let model = match selection {
            0 => ModelType::GEMINI,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid model selection",
                ))
            }
        };
        Ok(model)
    }

    fn update_prompt() -> io::Result<()> {
        let mut settings = settings::Settings::new().unwrap();
        let temp_file_path = "prompt_edit.txt";
        let initial_content = settings.get_prompt();
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

        let mut file = File::create(temp_file_path)?;

        file.write_all(initial_content.as_bytes())?;
        file.flush()?;

        SysCommand::new(editor)
            .arg(temp_file_path)
            .status()
            .expect("Failed to open the editor");

        let edited_content = fs::read_to_string(temp_file_path)?;
        settings.update_prompt(edited_content);
        settings.save().unwrap();

        fs::remove_file(temp_file_path)?;

        Ok(())
    }

    fn update_default_model(model: Option<String>) {
        let mut settings = settings::Settings::new().unwrap();

        let updated_model = match model {
            Some(default) => match default.to_uppercase().parse::<ModelType>().ok() {
                Some(value) => value,
                None => {
                    print!("Model not found\nuse \"aimit -d\" to select from supported models\n");
                    std::process::exit(0);
                }
            },
            None => Self::select_model().unwrap(),
        };
        settings.update_default_model(updated_model);
        settings.save().unwrap();
        std::process::exit(0);
    }

    fn set_api_key(matches: clap::ArgMatches) {
        let mut settings = settings::Settings::new().unwrap();

        let key = matches.get_one::<String>("key").unwrap().to_string();
        let model_flag = matches.contains_id("model");
        
        let cur_model = if model_flag {
            let selected_model = matches.get_one::<String>("model");
            match selected_model {
                None => Self::select_model().unwrap(),
                Some(value) => match value.to_uppercase().parse::<ModelType>().ok() {
                    Some(value) => value,
                    None => {
                        eprintln!("Model not found\nuse \"aimit -k <API_KEY> -m\" to select from supported models");
                        std::process::exit(1)
                    }
                },
            }
        } else {
            settings.get_default_model().clone()
        };


        settings.update_api_key(cur_model, Some(key));
        settings.save().unwrap();
        std::process::exit(0);
    }

    fn select_current_model(selected_model: Option<String>) -> ModelType {
        match selected_model {
            Some(default) => match default.to_uppercase().parse::<ModelType>().ok() {
                Some(value) => value,
                None => {
                    print!("Model not found\nuse \"aimit -m\" to select from supported models\n");
                    std::process::exit(1)
                }
            },
            None => Self::select_model().unwrap(),
        }
    }
}
