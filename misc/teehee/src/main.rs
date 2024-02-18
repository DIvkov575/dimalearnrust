use anyhow::Result;
use thiserror::Error;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args();
    let path = std::fs::Path::from(jargs[1]); // ensure exists

    if !path.exists() {
        std::fs::create_dir(path)
    } else if path.is_file() {
        return Err(MyError::PathWasFile.into())
    }

    let file = std::fs::OpenOptions::new().write(true).create(true).open(path.join("tmp"))?;
    file.write_all("teststring".as_bytes())?;

    Command::new("git").args(["commit", "-am", r#""test""#] ).output()?;
    Command::new("git").args(["push"]).output()?;

}

#[derive(Error, Debug)]
enum MyError {
    #[Error("Path provided was a file")]
    PathWasFile
}