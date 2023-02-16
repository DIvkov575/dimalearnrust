// awful wc

use std::fs;
use std::path::Path;
// use std::io::{self, BufRead, Write};
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if Path::new(&args[1]).exists() {
        let content = fs::read_to_string(&args[1]).unwrap();
        let c = content.matches(" ").count();
        println!("{}", c);

    } else {
        let c = &args[1].matches(" ").count();
        println!("{}", c);
    }
    // println!("{}", rs);

}

        // attempt at using crate but I can no figure it out
        // let reg: Regex = Regex::new(" ");
        // let restul= reg.find_iter(&content).count();
        // println!("{}", restul);

// get inputs
// iterate through inputs
// for each check if exist -> get content
// else -> get contetn
// content -> split by space or new line -> count words

