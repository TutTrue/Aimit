use config::settings;

mod cli;
mod config;
mod git;
mod models;
mod error;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), error::error::AimitError> {
    let model_type = cli::args::Cli::run();
    let settings = settings::Settings::new()?;

    let path = std::path::Path::new(".");
    let diff = git::diff::get_staged_diff(path)?;
    if diff == "" {
        return Err(error::error::AimitError::NoDiffFound);
    }
    let model = models::ModelFactory::create_model(
        model_type.clone(),
        settings.get_api_key(model_type)?.to_string(),
    );
    let res = model.generate_commit_message(&diff).await?;
    print!("{}", res);
    Ok(())
}
