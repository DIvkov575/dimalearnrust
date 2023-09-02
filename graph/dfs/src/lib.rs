use std::fmt;
use std::error::Error;

pub const END: i32 = 6;
#[derive(Debug)]
#[allow(dead_code)]
pub struct E {
    pub weight: i32,
    pub node: i32,
}

#[derive(Debug)]
pub struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

