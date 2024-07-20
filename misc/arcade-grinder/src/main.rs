use std::error::Error;
use std::fs::{copy, read_dir};
use std::path::Path;
use std::process::exit;
use regex::Regex;
use run_script::ScriptOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = Path::new("..").join("..").join("..").join("lpm1-arcade");
    let output_path = Path::new("..").join("..").join("..").join("website");

    get_commits(&input_path);
    exit(1);

    for file in read_dir(&output_path)?.map(|x| x.unwrap()) {
        if &file.file_name() != ".git" {
            // std::fs::remove_dir_all(file.path())?;
            println!("{:?} removed", &file.file_name())
        } else {
            println!("{:?}", file.file_name());
        }
    }

    for ifile_path in read_dir(&input_path)?.map(|x| x.unwrap()) {
        if &ifile_path.file_name() != ".git" {
            let ofile_path = output_path.join(&ifile_path.file_name());
            copy(&ifile_path.path(), &ofile_path)?;
            println!("{:?} copied to {:?}", &ifile_path.path(), ofile_path);
        } else {
            println!("{:?} not copied", ifile_path.file_name());
        }
    }
    Ok(())
}

fn get_commits(path: &Path) -> Vec<(String)> {
    let args = vec![path.to_str().unwrap().to_owned()];
    let (code, output, error) = run_script::run(
        r#"
        cd $1
        git checkout main
        git --no-pager log
         "#,
        &args,
        &ScriptOptions::new(),
    )
        .unwrap();

    println!("Exit Code: {}", code);
    // println!("Output: {}", output);
    println!("Error: {}", error);

    let regex = Regex::new(r"commit\s([a-f0-9]{40})\n(?:.|\n)*?\n\n\s*(.+)").unwrap();
    for capture in regex.captures_iter(&output) {
        println!("{}", std::iter::repeat("=").take(15).collect::<String>());
        if let Some(commit_id) = capture.get(1) {
            println!("{}", &commit_id.as_str());
        }
        if let Some(commit_message) = capture.get(2) {
            println!("{}", &commit_message.as_str());
        }
    }


    vec![]
}
