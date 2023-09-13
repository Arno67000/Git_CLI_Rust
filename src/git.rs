use chrono::{Duration, NaiveDateTime};
use git2::{BranchType, Oid, Repository};

#[derive(Debug, thiserror::Error)]
pub enum HandlerError {
    // #[error(transparent)]
    // IoError(std::io::Error),
    #[error(transparent)]
    CrossTerm(#[from] crossterm::ErrorKind),
    #[error(transparent)]
    Git(#[from] git2::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}
pub type Result<T, E = HandlerError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Branch {
    pub time: NaiveDateTime,
    pub commit_id: Oid,
    pub name: String,
    pub message: String,
    pub is_head: bool,
}

pub fn get_branches(repo: &Repository) -> Result<Vec<Branch>> {
    let mut branches = repo
        .branches(Some(BranchType::Local))?
        .map(|b| {
            let (branch, _) = b?;
            let name = String::from_utf8(branch.name_bytes()?.to_vec())?;
            let commit = branch.get().peel_to_commit()?;
            let commit_id = commit.id();
            let message = String::from_utf8(commit.message_bytes().to_vec())?;
            let time = commit.time();
            let offset = Duration::minutes(i64::from(time.offset_minutes()));
            let time = NaiveDateTime::from_timestamp_opt(time.seconds(), 0).unwrap() + offset;
            let is_head = branch.is_head();
            Ok(Branch {
                time,
                commit_id,
                name,
                message,
                is_head,
            })
        })
        .collect::<Result<Vec<Branch>>>()?;

    branches.sort_by_key(|branch| branch.time);
    Ok(branches)
}

pub fn delete_branch(repo: &Repository, branch: &Branch) -> Result<()> {
    let mut local_branch = repo.find_branch(&branch.name, BranchType::Local)?;
    local_branch.delete()?;
    Ok(())
}

pub fn get_repo() -> Result<Repository> {
    let repo = Repository::open_from_env()?;
    Ok(repo)
}
