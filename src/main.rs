use git::{get_branches, get_repo, Result};
use std::io::{stdin, stdout, Read};
use terminal::{communicate, disable_crossterm, display_local_info, enable_crossterm};

mod git;
mod terminal;

fn main() {
    let result = (|| -> Result<()> {
        enable_crossterm()?;

        let repo = get_repo()?;

        let mut stdout = stdout();
        let mut stdin = stdin().bytes();

        let branches = get_branches(&repo)?;

        display_local_info(&branches, &mut stdout)?;
        communicate(&branches, &mut stdout, &mut stdin, &repo)?;
        Ok(())
    })();

    disable_crossterm().ok();

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
    }
    println!("END");
}
