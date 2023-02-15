use std::fs::File;
use std::io::{self, BufRead};
use std::ffi::{OsString};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = OsString::from(&args[1]);
    // println!("{}", path.to_string());
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        println!("{}", line.unwrap());

    }
}
