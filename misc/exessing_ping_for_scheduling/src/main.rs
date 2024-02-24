use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = reqwest::blocking::get("http://127.0.0.1:3000");
    Ok(())
}