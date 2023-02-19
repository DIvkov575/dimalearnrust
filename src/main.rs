//unknown source of issue

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
    // recursive: bool,
    count: bool,
    // invert_match: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("grepr")
        .arg(
            Arg::with_name("pattern")
                .value_name("PATTERN")
                .help("Search 10+n")
                .required(true),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("."),
        )
        .arg(
            Arg::with_name("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("count")
                .short('c')
                .long("count")
                .help("Count occurrences")
                .takes_value(false),
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
        // recursive: matches.is_present("recursive"),
        count: matches.is_present("count"),
        // invert_match: matches.is_present("invert"),
    })
}

fn main() {
    let matches: Config = get_args().unwrap();
    // let file_name = matches.value_of("name").unwrap_or_default();
    // let paths: Vec<_> = matches.values_of("directory").unwrap_or_default().collect();

    // for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
    //     if file.metadata().unwrap().is_file() {
    //         if file.path().as_os_str().to_os_string().into_string().unwrap().contains(&file_name) {
    //             let content = std::fs::read_to_string(&file.path());
    //             // println!("");
    //         }
    //     }
    // }

}
