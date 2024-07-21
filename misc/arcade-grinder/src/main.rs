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
    gh_repo_name: String,
    slack_api_token: String,
    queue: VecDeque<(String, String)>,
}

impl Config {
    fn empty() -> Self {
        Self {queue: VecDeque::new(), input_path: "".to_string(), output_path: "".to_string(), gh_repo_name: "".to_string(), gh_username: "".to_string(), slack_api_token: "".to_string()}
    }

}

#[derive(Parser, Debug)]
enum Command {
    #[command(about="Init grinder")]
    Init,
    #[command(about="Deploy grinder")]
    Deploy,
}

impl Command {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Init => Ok(init().unwrap()),
            Command::Deploy=> Ok(deploy().unwrap()),
            // _ => Err(anyhow!("invalid command").into()),
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



static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| Path::new("ag_config.yaml").to_owned());

fn main() {
    Args::parse().run();
}

