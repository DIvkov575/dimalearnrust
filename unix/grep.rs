use clap::{self, Parser};
use regex::{Regex, RegexBuilder};
use std::collections::LinkedList;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pattern: String,
    #[arg(short, long)]
    files: Vec<String>,
    #[arg(short, long)]
    insensitive: bool,
    #[arg(short, long)]
    recursive: bool,
    #[arg(short, long)]
    count: bool,
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let pattern = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.insensitive)
        .build()
        .map_err(|_| format!("Invalid pattern \"{}\"", &args.pattern))
        .unwrap();
    let mut count = 0;
    for path_a in args.files {
        if fs::metadata(&path_a).unwrap().is_file() {
            let lines: Vec<String> = BufReader::new(File::open(&path_a).unwrap())
                .lines()
                .map(|x| x.unwrap())
                .collect();

            // let mut pat_match_locations: LinkedList<usize> = LinkedList::new();
            let mut pat_match_locations: Vec<usize> = Vec::new();
            pat_match_locations.push(0);
            let content = fs::read_to_string(&path_a).unwrap();

            for pat_match in pattern.captures_iter(&content) {
                let nl_count: usize = content[0..pat_match.get(0).unwrap().start()]
                    .matches('\n')
                    .count();
                if args.verbose {
                    println!("{}, {}, line: {}", &path_a, nl_count, lines[nl_count])
                } else {
                    println!("{}, line: {}", &path_a, nl_count)
                }
                count += 1;
            }
        } else {
            if args.recursive {
                for path_b in WalkDir::new(&path_a)
                    .into_iter()
                    .filter_map(|file| file.ok())
                {
                    if path_b.metadata().unwrap().is_file() {
                        println!("{}", path_b.path().display());
                        let lines: Vec<String> = BufReader::new(File::open(path_b.path()).unwrap())
                            .lines()
                            .map(|x| x.unwrap())
                            .collect();
                        let mut pat_match_locations: LinkedList<usize> = LinkedList::new();
                        pat_match_locations.push_back(0);
                        let contents = std::fs::read_to_string(path_b.path()).unwrap();

                        for pat_match in pattern.captures_iter(&contents) {
                            let nl_count = contents[*pat_match_locations.front().unwrap()
                                ..pat_match.get(0).unwrap().start()]
                                .matches('\n')
                                .count();
                            if args.verbose {
                                println!("{}, {}, line: {}", &path_a, nl_count, lines[nl_count])
                            } else {
                                println!("{}, line: {}", &path_a, nl_count)
                            }
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    if args.count {
        println!("Total number of occurrences: {}", count);
    }
}
