use std::fmt::format;
use std::io::Write;
use std::path::Path;
use thiserror::{Error, self};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path;
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        path = Path::new(&args[1]);
    } else {
        path = Path::new("tmp");
    }

    let panic_msg = format!("file {} already exists", path.to_str().unwrap());
    if path.exists() { panic!("{}", panic_msg) }

    let mut file = std::fs::OpenOptions::new().write(true).append(true).create(true).open(path)?;
    file.write_all("A".as_bytes())?;

    Command::new("git").args(["add", "tmp"]).output()?;
    Command::new("git").args(["commit", "-am", r#""test""#] ).output()?;
    Command::new("git").args(["push"]).output()?;
    Command::new("git").args(["reset", "HEAD@{1}"]).output()?;
    Command::new("git").args(["push", "--force"]).output()?;

    if path.exists() { std::fs::remove_file(&path)?; }

    Ok(())
}
