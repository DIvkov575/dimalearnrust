use anyhow::Result;
use std::error::Error;
use std::{fs, io, path};
use std::fs::read_dir;
use std::io::read_to_string;
use std::path::Path;
use std::process::{Command, Output};
use anyhow::Context;
use crate::{Config, CONFIG_PATH};

pub fn deploy() -> Result<(), Box<dyn Error>> {
    let file = fs::File::options()
        .create(true)
        .write(true)
        .read(true)
        .open::<&path::PathBuf>(&CONFIG_PATH)
        .with_context(|| format!("Error opening or creating file {} {}", file!(), line!()))?;
    let config_str = read_to_string(&file).with_context(|| format!("Error reading config file {} {}", file!(), line!()))?;
    let config: Config = serde_yaml::from_str(&config_str).with_context(|| format!("Error deserializing string {} {}", file!(), line!()))?;


    let input_path = path::Path::new(&config.input_path);
    let output_path = path::Path::new(&config.output_path);

    checkout(&input_path, &config.queue[0].0)?;
    copy_files(&input_path, &output_path)?;
    gacp(&output_path, &config.queue[0].1)?;

    let commmit_url = gh_commit_url(&config, &config.queue[0].0);
    println!("\n\n{}", commmit_url);

    Ok(())

}


fn checkout(path: &Path, id: &str) -> Result<(), io::Error> {
    let args = vec!["-C", path.to_str().unwrap(), "checkout", id];
    let output = Command::new("git").args(args).output()?;
    println!("{:?}", output);
    Ok(())
}


fn copy_files(input_path: &Path, output_path: &Path) -> Result<()>{
    for file in read_dir(&output_path)?.map(|x| x.unwrap()) {
        if &file.file_name() != ".git" {
            // fs::remove_dir_all(file.path())?;
            Command::new("rm")
                .arg("-r")
                .arg(&file.path())
                .output()?;
            println!("{:?} removed", &file.file_name())
        } else {
            println!("{:?}", file.file_name());
        }
    }

    for ifile_path in read_dir(&input_path)?.map(|x| x.unwrap()) {
        if &ifile_path.file_name() != ".git" {
            let ofile_path = output_path.join(&ifile_path.file_name());

            Command::new("cp")
                .arg("-r")
                .arg(&ifile_path.path())
                .arg(&ofile_path)
                .output()?;

            println!("{:?} copied to {:?}", &ifile_path.path(), ofile_path);
        } else {
            println!("{:?} not copied", ifile_path.file_name());
        }
    }

    Ok(())
}

fn gacp(path: &Path, message: &str) -> Result<()> {
    // git add commit & push

    //add
    let args = vec!["-C", path.to_str().unwrap(), "add", "-A"];
    let output = Command::new("git").args(args).output()?;
    println!("{:?}", output);

    //commit
    let formatted_message = format!(r#"'{}'"#, message);
    let args = vec!["-C", path.to_str().unwrap(), "commit", "-am", &formatted_message];
    let output = Command::new("git").args(args).output()?;
    println!("{:?}", output);

    // push
    // let output = Command::new("git").arg("push").output()
    //     .with_context(|| format!("{:?} @ {} {}", output, file!(), line!()))?;

    Ok(())
}

fn gh_commit_url(config: &Config, commit_id: &str) -> String {
    format!("https://github.com/{}/{}/commit/{}", &config.gh_username, &config.gh_repo_name, commit_id)
}