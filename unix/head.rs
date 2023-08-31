use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len()-1;
    let mut line_num: u32 = 10;
    let mut ctr = 0;
    if num_args > 1 {line_num = args[2].parse::<u32>().unwrap();}

    let file = File::open(&args[1]).unwrap();
    let buf_file = io::BufReader::new(file);
    // let num_lines = lines.len();
    let lines = buf_file.lines();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // println!("{}", num_lines);


// make tail

    for line in lines {
        if ctr < line_num {
            // println!("{}", line.unwrap());
            writeln!(handle, "{}", line.unwrap());
            ctr += 1;
        } else {
            std::process::exit();
        }


    }
}

// fn read_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let data = fs::read(filepath)?;

// }
