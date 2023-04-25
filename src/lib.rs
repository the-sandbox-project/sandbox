mod editor;
mod config;
mod args;
mod environment;
mod search;
mod install;

use args::SandboxArgs;
use clap::Parser;
use install::install_environment;
use search::search;

use crate::environment::setup_environment;

pub fn run() { 
    let args = SandboxArgs::parse();

    if !args.search.is_empty() {
        search(args.search)
    }

    if !args.beach_type.is_empty() {
        setup_environment(args.beach_type)
    }

    if !args.install.is_empty() {
        install_environment(args.install)
    }
}
