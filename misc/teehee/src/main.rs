use std::io::Write;
use std::path::Path;
use anyhow::Result;
use thiserror::{Error, self};
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // let path = Path::new(&args[1]);
    let path = Path::new("tmp");

    if !path.exists() {
        std::fs::create_dir(path)?;
    } else if path.is_file() {
        return Err(MyError::PathWasFile.into());
    }

    let mut file = std::fs::OpenOptions::new().write(true).append(true).create(true).open(path.join("tmp"))?;
    file.write_all("teststring".as_bytes())?;

    Command::new("git").args(["add", "-A"]).output()?;
    Command::new("git").args(["commit", "-am", r#""test""#] ).output()?;
    Command::new("git").args(["push"]).output()?;

    Command::new("git").args(["reset", "HEAD@{1}"]).output()?;
    Command::new("git").args(["push", "--force"]).output()?;

    Ok(())

}

#[derive(Error, Debug)]
#[error("Dataflow Error")]
enum MyError {
    #[error("Path provided was a file")]
    PathWasFile
}