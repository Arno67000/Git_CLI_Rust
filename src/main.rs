use std::io::{stdin, stdout, Read, Stdin, Write};

use git2::{BranchType, Repository};

#[derive(Debug, thiserror::Error)]
enum HandlerError {
    #[error(transparent)]
    IoError(std::io::Error),

    #[error(transparent)]
    CrossTermError(#[from] crossterm::ErrorKind),
    #[error(transparent)]
    GitError(#[from] git2::Error),
}

fn main() -> Result<(), HandlerError> {
    crossterm::terminal::enable_raw_mode()?;

    let repo = Repository::open_from_env()?;
    for branch in repo.branches(Some(BranchType::Local))? {
        let (branch, _) = branch?;
        let name = branch.name_bytes()?;
    }

    let mut stdout = stdout();
    let mut stdin = stdin().bytes();

    loop {
        write!(stdout, "Type smthg > ")?;
        stdout.flush()?;

        let byte = match stdin.next() {
            Some(byte) => byte?,
            None => break,
        };
        let c = char::from(byte);
        if c == 'q' {
            break;
        }

        write!(stdout, "You typed '{}'\n\r", c)?;
        stdout.flush()?;
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
