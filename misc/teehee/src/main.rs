use std::io::Write;
use std::path::Path;
use anyhow::Result;
use thiserror::{Error, self};
use std::process::Command;

fn main() -> Result<()> {
    let path = Path::new("tmp");
    if path.exists() { panic!("file 'tmp' already exists") }

    let mut file = std::fs::OpenOptions::new().write(true).append(true).create(true).open(path.join("tmp"))?;
    file.write_all("A".as_bytes())?;

    Command::new("git").args(["add", "-A"]).output()?;
    Command::new("git").args(["commit", "-am", r#""test""#] ).output()?;
    Command::new("git").args(["push"]).output()?;
    Command::new("git").args(["reset", "HEAD@{1}"]).output()?;
    Command::new("git").args(["push", "--force"]).output()?;

    if path.exists() { std::fs::remove_file(&path)?; }

    Ok(())
}
