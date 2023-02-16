use std::fs::File;
use std::io::{self, BufRead, Error};
use std::env;
use std::result::Result;

fn main() {
    let args: Vec<String> = env::args().collect();


    let mut line_num: usize = 10;
    if args.len() > 2 { line_num = args[2].parse::<usize>().unwrap(); }
    let buf_file = io::BufReader::new(File::open(&args[1]).unwrap());
    let lines: Vec<Result<String, Error>> = buf_file.lines().collect();
    let mut start_line: usize; 
    let end_line: usize = lines.len(); 

    if end_line-line_num < 0 {
        start_line = 0; 
    } else {
        start_line = end_line-line_num;
    }
    for line in &lines[start_line..end_line] {
        match line {
            Ok(cont) => println!("{:?}", cont),
            Err(e) => panic!("asldf;lsdjfk {:?}", e),
        };
        // Ok(())
    }
}
