use std::error::Error;
use std::{fs,path};
use std::fmt::format;
use std::io::Write;
use anyhow::Context;
use crate::{Config, CONFIG_PATH};


pub fn init() -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::options()
        .create(true)
        .write(true)
        .read(true)
        .open::<&path::PathBuf>(&CONFIG_PATH)
        .with_context(|| format!("Error opening or creating file {}{}", file!(), line!()))?;

    let config = Config::empty();
    let config_str = serde_yaml::to_string(&config)
        .with_context(|| format!("Error serializing empty config {}{}", file!(), line!()))?;

    file.write_all(config_str.as_bytes())
        .with_context(|| format!("Error writing config {}{}", file!(), line!()))?;

    println!("{} created. Fill fields before continuing", CONFIG_PATH.to_str().unwrap());
    Ok(())
}