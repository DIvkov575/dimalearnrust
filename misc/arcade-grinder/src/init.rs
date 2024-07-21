use std::error::Error;
use std::fs::File;
use crate::CONFIG_PATH;

pub fn init() -> Result<(), Box<dyn Error>> {
    let file = File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(CONFIG_PATH.as_ref())?;



    Ok(())
}