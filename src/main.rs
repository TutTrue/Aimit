use std::process::exit;

use config::settings;

mod cli;
mod config;
mod git;
mod models;

#[tokio::main]
async fn main() {
    let model_type = cli::args::Cli::run();
    let settings = settings::Settings::new().unwrap();

    let path = std::path::Path::new(".");
    let diff = git::diff::get_staged_diff(path).unwrap();
    if diff == "" {
        print!("There is no diff\n");
        exit(1)
    }
    let model = models::ModelFactory::create_model(
        model_type.clone(),
        settings.get_api_key(model_type).unwrap().to_string(),
    );
    let res = model.generate_commit_message(&diff).await;
    match res {
        Ok(res) => print!("{}", res),
        Err(res) => print!("err: {:?}", res),
    }
}
