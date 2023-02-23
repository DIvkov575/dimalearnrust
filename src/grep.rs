//unknown source of issue
// uses clap 2.33
use std::collections::LinkedList;
use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem,
};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    verbose: u8,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("grepr")
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("insensitive")
                .short("i")
                .long("insensitive")
                .help("Case-insensitive")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Recursive search")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Count occurrences")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .value_name("verbose")
                .help("verbose (int)")
                .default_value("0")
                .required(false),
        )
        .get_matches();


    let pattern = matches.value_of("pattern").unwrap();
    let pattern = RegexBuilder::new(pattern)
        .case_insensitive(matches.is_present("insensitive"))
        .build()
        .map_err(|_| format!("Invalid pattern \"{}\"", pattern))?;

    Ok(Config {
        pattern,
        files: matches.values_of_lossy("files").unwrap(),
        recursive: matches.is_present("recursive"),
        count: matches.is_present("count"),
        verbose: matches.value_of("verbose").unwrap().parse::<u8>().unwrap(),
    })
}

fn main() {
    let matches: Config = get_args().unwrap();
    let mut count = 0;

    for path_a in matches.files {
        // println!("{}", &dir_path_a);
        
        if fs::metadata(&path_a).unwrap().is_file() {
                let lines: Vec<String> = BufReader::new(File::open(&path_a).unwrap()).lines().map(|x| x.unwrap()).collect();

                let mut pat_match_locations: LinkedList<usize> = LinkedList::new();
                pat_match_locations.push_back(0);
                let content = fs::read_to_string(&path_a).unwrap();

                for pat_match in matches.pattern.captures_iter(&content) {
                    let nl_count = content[*pat_match_locations.front().unwrap()..pat_match.get(0).unwrap().start()].matches('\n').count();
                    match matches.verbose {
                        0 => println!("{}, line: {}", &path_a, nl_count),
                        1 => println!("{}, {}, line: {}", &path_a, nl_count, lines[nl_count]),
                        _ => panic!("Incorect values passed for verbose, please pass a value from (0 - 1)"),
                    } 
                    count += 1;

                }
        } else {
            if matches.recursive {
                for path_b in WalkDir::new(&path_a).into_iter().filter_map(|file| file.ok()) {
                    if path_b.metadata().unwrap().is_file() {
                        println!("{}", path_b.path().display());
                        let  lines: Vec<String> = BufReader::new(File::open(path_b.path()).unwrap()).lines().map(|x| x.unwrap()).collect();
                        let mut pat_match_locations: LinkedList<usize> = LinkedList::new();
                        pat_match_locations.push_back(0);
                        let contents = std::fs::read_to_string(path_b.path()).unwrap();

                        for pat_match in matches.pattern.captures_iter(&contents) {
                            let nl_count = contents[*pat_match_locations.front().unwrap()..pat_match.get(0).unwrap().start()].matches('\n').count();
                            match matches.verbose {
                                0 => println!("{:#?}, line: {}", &path_b.path(), nl_count),
                                1 => println!("{:#?}, {}, line: {}", &path_b.path(), nl_count, lines[nl_count]),
                                _ => panic!("Incorect values passed for verbose, please pass a value from (0 - 1)"),
                            } 
                            count += 1;
                        }
                    }
                } 
            }
        }
    }
    if matches.count {println!("Total number of occurrences: {}", count);}
}
