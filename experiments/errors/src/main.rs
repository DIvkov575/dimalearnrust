use std::fmt::{Display, Formatter};
use anyhow::{Context, Result};


fn main() -> Result<()> {
    // fn main() -> result<(), box<dyn std::error::error>> {
    a()?;


    Ok(())
}
















fn a() -> Result<()> {
    Err(MyError {message: "womp womp".to_string()}.into())
}

#[derive(Debug)]
struct MyError {
    message: String
}

impl Display for MyError {
    #[track_caller]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} @ {} {}", self.message,std::panic::Location::caller().file(), std::panic::Location::caller().line())?;
        Ok(())
    }
}

impl std::error::Error for MyError {}

