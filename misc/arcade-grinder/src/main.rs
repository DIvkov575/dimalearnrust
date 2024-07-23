mod init;
mod deploy;

use serde::{Serialize, Deserialize};
use serde_yaml;
use std::collections::VecDeque;
use std::error::Error;
use std::path::{Path, PathBuf};
use clap::Parser;
use once_cell::sync::Lazy;
use crate::deploy::deploy;
use crate::init::init;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    input_path: String,
    output_path: String,
    gh_username: String,
}

impl Config {
    fn empty() -> Self {
        Self { input_path: "".to_string(), output_path: "".to_string(), gh_username: "".to_string()}
    }

}

#[derive(Parser, Debug)]
enum Command {
    #[command(about="Init grinder")]
    Init {
        #[arg(short, long)]
        input_path: String,
        #[arg(short, long)]
        output_path: String,
        #[arg(short, long)]
        gh_username: Option<String>

    },
    #[command(about="Deploy grinder")]
    Deploy {
        #[arg(short, long)]
        commit_id: String,
        #[arg(short, long)]
        message: Option<String>,
    },
}

impl Command {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Init {input_path, output_path, gh_username}=> Ok(init(input_path, output_path, gh_username).unwrap()),
            Command::Deploy {commit_id, message}=> Ok(deploy(commit_id, message).unwrap()),
            _ => Err("invalid command".into()),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None,)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}
impl Args { pub fn run(self) { self.command.run().unwrap(); } }


static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| home::home_dir().unwrap().join(".ag_config.yaml"));

fn main() {
    Args::parse().run();
}

