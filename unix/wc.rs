use clap::{App, Arg};
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let matches = App::new("My Test Program")
        .about("WC in rust v2")
        .arg(
            Arg::with_name("file_path")
                .short('f')
                .long("files")
                .takes_value(true)
                .help(""),
        )
        .get_matches();

    let file_path = matches.value_of("file_path").unwrap_or("input.txt");
    let file_content = std::fs::read_to_string(&file_path).unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(r" \S").unwrap();
    }
    let ctr = RE.captures_iter(&file_content).count();
    println!("{}", ctr);

    // println!("{}", matches.value_of("a").unwrap_or("asdf"));

    // let num_str = matches.value_of("num");
    // match num_str {
    //     None => println!("No idea what your favorite number is."),
    //     Some(s) => {
    //         match s.parse::<i32>() {
    //             Ok(n) => println!("Your favorite number must be {}.", n + 5),
    //             Err(_) => println!("That's not a number! {}", s),
    //         }
    //     }
    // }
}
