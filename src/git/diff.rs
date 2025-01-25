use git2::Repository;
use crate::error::error::AimitError;
use std::path::Path;


pub fn get_staged_diff(repo_path: &Path) -> Result<String, AimitError> {
    let repo = Repository::open(repo_path).map_err(|_| AimitError::RepoNotFound)?;
    let head = repo.head().map_err(|_| AimitError::GitError)?.peel_to_commit().map_err(|_| AimitError::GitError)?;
    let head_tree = head.tree().map_err(|_| AimitError::GitError)?;
    let mut opts = git2::DiffOptions::new();
    let diff = repo.diff_tree_to_index(Some(&head_tree), None, Some(&mut opts)).map_err(|_| AimitError::GitError)?;
    let mut diff_text = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        diff_text.push_str(std::str::from_utf8(line.content()).unwrap());
        true
    }).map_err(|_| AimitError::GitError)?;
    Ok(diff_text)
}
