use std::process::exit;

mod git;
mod models;
mod config;

#[tokio::main]
async fn main() {
    let path = std::path::Path::new(".");
    let diff = git::diff::get_staged_diff(path).unwrap();
    if diff == "" {
        print!("There is no diff\n");
        exit(1)
    }
    let gemini =
        models::ModelFactory::create_model(models::ModelType::GEMINI, "api-key-123".to_string());
    let res = gemini.generate_commit_message(&diff).await;
    match res {
        Ok(res) => print!("ok: {}", res),
        Err(res) => print!("err: {:?}", res),
    }
}
