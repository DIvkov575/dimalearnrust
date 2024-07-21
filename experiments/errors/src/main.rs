use std::fmt::{Display, Formatter};
use anyhow::{Context, Result};


fn main() -> Result<()> {
    // fn main() -> result<(), box<dyn std::error::error>> {
    //     a()?;
    //     b();
    c();


    Ok(())
}



fn b() {
    println!("{:?}", std::panic::Location::caller());
}

#[track_caller]
fn c() {
    d();
}
#[track_caller]
fn d() {
    a().unwrap();
}











fn a() -> Result<()> {
    Err(MyError {message: "womp womp".to_string()}.into())
}

#[derive(Debug)]
struct MyError {
    message: String
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} @ {} {}", self.message,std::panic::Location::caller().file(), std::panic::Location::caller().line())?;
        Ok(())
    }
}

impl std::error::Error for MyError {}

