use anyhow::{bail, Result};
use std::error::Error;
use std::{fs, io, path};
use std::fs::read_dir;
use std::io::read_to_string;
use std::mem::MaybeUninit;
use std::path::Path;
use std::process::{Command, Output};
use anyhow::Context;
use regex::Regex;
use crate::{Config, CONFIG_PATH};

pub fn deploy(commit_id: String, message_option: Option<String>) -> Result<(), Box<dyn Error>> {
    let file = fs::File::options()
        .create(true)
        .write(true)
        .read(true)
        .open::<&path::PathBuf>(&CONFIG_PATH)
        .with_context(|| format!("Error opening or creating file {} {}", file!(), line!()))?;
    let config_str = read_to_string(&file).with_context(|| format!("Error reading config file {} {}", file!(), line!()))?;
    let config: Config = serde_yaml::from_str(&config_str).with_context(|| format!("Error deserializing string {} {}", file!(), line!()))?;

    let input_path = Path::new(&config.input_path);
    let output_path = Path::new(&config.output_path);


    checkout(&input_path, &commit_id)?;
    copy_files(&input_path, &output_path)?;

    if let Some(commit_message) = message_option {
        gacp(&output_path, &commit_message)?;
    } else {
        let commit_message = get_message_from_id(&input_path, &commit_id)?;
        gacp(&output_path, &commit_message)?;
    }

    let commit_url = gh_commit_url(&config, &output_path)?;
    println!("\n\n{}", commit_url);

    Ok(())

}


fn get_message_from_id(input_path: &Path, id: &str) -> Result<String> {
    let args = vec!["-C", input_path.to_str().unwrap(), "log"];
    let output = Command::new("git").args(&args).output()
        .with_context(|| format!("error get git log from input dir {} {}", file!(), line!()))?;
    println!("output: {:?}", &output);
    let parsed_output = String::from_utf8(output.stdout)?;

    let regex = Regex::new(r"commit\s([a-f0-9]{40})\n(?:.|\n)*?\n\n\s*(.+)").unwrap();

    for capture in regex.captures_iter(&parsed_output) {
        if capture.get(1).unwrap().as_str() == id {
            return Ok(capture.get(2).unwrap().as_str().to_owned());
        }
    }
    bail!("message w/ specified commit not found");
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
    let args = vec!["-C", path.to_str().unwrap(), "push"];
    let output = Command::new("git").args(args).output()
        .with_context(|| format!("{:?} @ {} {}", output, file!(), line!()))?;

    Ok(())
}

fn gh_commit_url(config: &Config, output_path: &Path) -> Result<String> {
    // get all logs
    let args = vec!["-C", output_path.to_str().unwrap(), "log"];
    let output = Command::new("git").args(&args).output()
        .with_context(|| format!("error get git log from output dir {} {}", file!(), line!()))?;
    println!("output: {:?}", &output);
    let parsed_output = String::from_utf8(output.stdout)?;


    // find log w/ specified id
    let mut commit_id = "";
    if let Some(capture)  = Regex::new(r"commit ([a-f0-9]{40})")
        .unwrap()
        .captures(&parsed_output) {
        commit_id = capture.get(1).unwrap().as_str();
    }

    // format data
    let repo_name = output_path.file_name().unwrap().to_str().unwrap();
    Ok(format!("https://github.com/{}/{}/commit/{}", &config.gh_username, repo_name, &commit_id))
}