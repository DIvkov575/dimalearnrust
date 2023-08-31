// uniq
use clap::{Parser};
use std::{
    fs::{File},
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   file: String,
}

fn main() {
    let args = Args::parse();
    let mut lines = BufReader::new(File::open(&args.file).unwrap()).lines().map(|x| x.unwrap());
    let mut prev_line = lines.next().unwrap();
    for line in lines {
            if &prev_line != &line {
                println!("{}", &prev_line);
            }
            prev_line = line;
    }
}

