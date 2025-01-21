mod git;

fn main() {
    let path = std::path::Path::new(".");
    let diff = git::diff::get_staged_diff(path);
    match diff {
        Ok(diff) => println!("{}", diff),
        Err(e) => {println!("Error: {}", e)},
    }
}
