use clap::{Arg, App};

fn main() {
    let matches = App::new("app app app")
        .version("0.1.0")
        .author("Dmitriy Ivkov <dmitriy@ivkov.net>")
        .about("ls")
        .arg(Arg::with_name("directory")
                 .short('d')
                 .long("dir
                 .takes_value(true)
                 .help("asdf"))
            .get_matches();
    let path = matches.value_of("directory").unwrap_or_default();
    for file in std::fs::read_dir(&path).unwrap() {
        println!("{}", file.unwrap().path().display());
    }

}
