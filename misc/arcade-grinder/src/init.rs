use std::error::Error;
use std::{fs,path};
use std::fmt::format;
use std::io::Write;
use anyhow::Context;
use crate::{Config, CONFIG_PATH};


pub fn init(input_path: String, output_path: String, gh_username: Option<String>) -> Result<(), Box<dyn Error>> {
    // create empty version of struct if file doesn't exist
    if !CONFIG_PATH.exists() {
        let config_empty = Config::empty();
        let mut file = fs::File::options()
            .create(true)
            .write(true)
            .read(true)
            .open::<&path::PathBuf>(&CONFIG_PATH)
            .with_context(|| format!("Error opening or creating file {}{}", file!(), line!()))?;
        let config_str = serde_yaml::to_string(&config_empty)
            .with_context(|| format!("Error serializing empty config {}{}", file!(), line!()))?;
        file.write_all(config_str.as_bytes())
            .with_context(|| format!("Error writing config {}{}", file!(), line!()))?;
    }
    let file_read = fs::File::options()
        .read(true)
        .open::<&path::PathBuf>(&CONFIG_PATH)
        .with_context(|| format!("Error opening or creating file {}{}", file!(), line!()))?;

    let mut config: Config = serde_yaml::from_reader(&file_read).with_context(|| format!("deserializing existing config @ {} {}", file!(), line!()))?;
    if gh_username.is_some() { config.gh_username = gh_username.unwrap()}
    config.input_path = input_path;
    config.output_path= output_path;
    let config_str = serde_yaml::to_string(&config)
        .with_context(|| format!("Error serializing empty config {}{}", file!(), line!()))?;


    fs::remove_file::<&path::PathBuf>(&CONFIG_PATH).with_context(|| format!("attemptign to clear config file"))?;

    let mut file = fs::File::options()
        .create(true)
        .write(true)
        .read(true)
        .open::<&path::PathBuf>(&CONFIG_PATH)
        .with_context(|| format!("Error opening or creating file {}{}", file!(), line!()))?;
    file.write_all(config_str.as_bytes())
        .with_context(|| format!("Error writing config {}{}", file!(), line!()))?;

    println!("{} created. Fill fields before continuing", CONFIG_PATH.to_str().unwrap());
    Ok(())
}