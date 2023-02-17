use std::env;
use std::fs;

fn main() {
    // let matches = App::new("app app app")
    //     .version("0.1.0")
    //     .author("Dmitriy Ivkov <dmitriy@ivkov.net>")
    //     .about("ls")
    //     .arg(Arg::with_name("directory")
    //              .short('d')
    //              .long("dir")
    //              .takes_value(true)
    //              .help("asdf"))
    //     .get_matches();
    // let path = matches.value_of("directory").unwrap_or_default();
    let args: Vec<String> = env::args().collect();
    let num_args = args.len()-1;

    // let paths = fs::read_dir("./").unwrap();

    // for path in paths {
        // println!("Name: {}", path.unwrap().path().display())
        // if fs::metadata(&path).unwrap().is
    // }
    let path = String::from("./");
    subpaths(path);
}

fn subpaths(path: String) {
    // let paths: Vec<String> = fs::read_dir(&path).unwrap().collect();
    let paths: Vec<fs::DirEntry> = fs::read_dir(&path)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();


    for path in paths {
        // println!("Name: {}", path.unwrap().path().display())
        let meta_path= fs::metadata(path).unwrap();
        if meta_path.is_file() {
            // println!("{}", meta_path.display());
        }
    }
}
