use git2::Repository;
use std::path::Path;

pub fn get_staged_diff(repo_path: &Path) -> Result<String, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?.peel_to_commit()?;
    let head_tree = head.tree()?;
    let mut opts = git2::DiffOptions::new();
    let diff = repo.diff_tree_to_index(Some(&head_tree), None, Some(&mut opts))?;
    let mut diff_text = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        diff_text.push_str(std::str::from_utf8(line.content()).unwrap());
        true
    })?;
    Ok(diff_text)
}
