use crossterm::{
    execute,
    style::{Attribute, Color, Print, SetAttribute, SetForegroundColor},
};
use git::{delete_branch, get_branches, Branch, HandlerError, Result};
use git2::Repository;
use std::io::{stdin, stdout, Bytes, Read, Stdin, Stdout, Write};

mod git;

enum Action {
    Show,
    Keep,
    Delete,
    Quit,
    Help,
}
impl TryFrom<char> for Action {
    type Error = HandlerError;
    fn try_from(value: char) -> Result<Self> {
        match value {
            'k' => Ok(Action::Keep),
            'q' => Ok(Action::Quit),
            's' => Ok(Action::Show),
            'd' => Ok(Action::Delete),
            _ => Ok(Action::Help),
        }
    }
}

enum Validate {
    Accept,
    Refuse,
}
impl TryFrom<char> for Validate {
    type Error = HandlerError;
    fn try_from(value: char) -> Result<Self> {
        match value {
            'y' => Ok(Validate::Accept),
            'n' => Ok(Validate::Refuse),
            _ => Ok(Validate::Refuse),
        }
    }
}

fn main() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let repo = Repository::open_from_env()?;

    let mut stdout = stdout();
    let mut stdin = stdin().bytes();

    let branches = get_branches(&repo)?;
    communicate(&branches, &mut stdout, &mut stdin, &repo)?;

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn communicate(
    branches: &Vec<Branch>,
    stdout: &mut Stdout,
    stdin: &mut Bytes<Stdin>,
    repo: &Repository,
) -> Result<()> {
    for branch in branches {
        execute!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print("\r\nBranch: "),
            SetAttribute(Attribute::Bold),
            Print(&branch.name),
            SetAttribute(Attribute::Reset),
            SetForegroundColor(Color::Blue),
            Print(" -> last_commit: "),
            Print(branch.time),
            Print("\r\n"),
        )?;
        loop {
            execute!(
                stdout,
                SetAttribute(Attribute::Bold),
                Print("(s,k,d,?,q) > "),
                SetAttribute(Attribute::Reset),
                SetForegroundColor(Color::Blue)
            )?;
            stdout.flush()?;

            let byte = match stdin.next() {
                Some(byte) => byte?,
                None => break,
            };
            let c = char::from(byte);
            let action = Action::try_from(c)?;
            match action {
                Action::Quit => {
                    write!(stdout, "{}\r\n", c)?;
                    return Ok(());
                }

                Action::Show => {
                    write!(stdout, "{}\r\n", c)?;
                    write!(
                        stdout,
                        "SHA1 : '{}' \r\nmessage: {}\r\n",
                        branch.commit_id, branch.message
                    )?;
                    stdout.flush()?;
                    continue;
                }

                Action::Keep => {
                    write!(stdout, "{}\r\n", c)?;
                    stdout.flush()?;
                    break;
                }

                Action::Delete => {
                    write!(stdout, "{}\r\n", c)?;
                    write!(stdout, "Are you sure you want to ")?;
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Red),
                        Print("delete"),
                        SetForegroundColor(Color::Blue),
                        Print(" branch : ")
                    )?;
                    execute!(
                        stdout,
                        SetAttribute(Attribute::Bold),
                        Print(&branch.name),
                        Print("\r\n(y,n) > "),
                        SetAttribute(Attribute::Reset),
                        SetForegroundColor(Color::Blue),
                    )?;
                    stdout.flush()?;
                    let byte = match stdin.next() {
                        Some(byte) => byte?,
                        None => break,
                    };
                    let c = char::from(byte);
                    let validation = Validate::try_from(c)?;
                    match validation {
                        Validate::Accept => {
                            write!(stdout, "{}\r\n", c)?;
                            delete_branch(repo, branch)?;
                            write!(stdout, "Branch succesfully deleted\r\n")?;
                        }
                        Validate::Refuse => {
                            write!(stdout, "{}\r\n", c)?;
                            write!(stdout, "Delete was aborted\r\n")?;
                            continue;
                        }
                    }
                    break;
                }

                Action::Help => {
                    write!(stdout, "{}\r\n", c)?;
                    execute!(stdout, SetForegroundColor(Color::DarkYellow))?;
                    write!(stdout, "Commands details:\r\n")?;
                    write!(stdout, "'s' => Show last commit details\r\n")?;
                    write!(stdout, "'d' => Delete the banch\r\n")?;
                    write!(stdout, "'k' => Keep the branch\r\n")?;
                    write!(stdout, "'q' => Quit the program\r\n")?;
                    write!(stdout, "'?' => Show this help\r\n")?;
                    stdout.flush()?;
                    execute!(stdout, SetForegroundColor(Color::Blue))?;
                    continue;
                }
            }
        }
    }
    Ok(())
}
