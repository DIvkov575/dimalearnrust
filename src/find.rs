#![feature(option_result_contains)]
extern crate walkdir;
use walkdir::WalkDir;
use clap::{Arg, App};

fn main() {
    let matches = App::new("app app app")
        .arg(Arg::with_name("directory")
                .short('D')
                .long("dir")
                .takes_value(true))
        .arg(Arg::with_name("name")
                .required(true)
                .short('n')
                .long("name")
                .takes_value(true)
                .min_values(1))
        .get_matches();
    let file_name = matches.value_of("name").unwrap_or_default();
    let paths: Vec<_> = matches.values_of("directory").unwrap_or_default().collect();
    for path in paths {
        for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                if file.path().as_os_str().to_os_string().into_string().unwrap().contains(&file_name) {
                    println!("{:#?}", file.path().as_os_str());
                }
            }
        }
    } 
}
