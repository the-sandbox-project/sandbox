mod editor;
mod config;
mod args;
mod environment;
mod search;
mod install;

use args::SandboxArgs;
use clap::Parser;
use install::install_environment;
use search::search_environment;

pub fn run() { 
    let args = SandboxArgs::parse();
    // setup_environment(args.beach_type)
    // search_environment("minimal".to_string())
    install_environment("go-min".to_string())

}
