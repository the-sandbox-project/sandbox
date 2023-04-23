mod editor;
mod config;
mod args;
mod environment;
mod search;

use args::SandboxArgs;
use clap::Parser;
use environment::setup_environment;

pub fn run() { 
    let args = SandboxArgs::parse();
    setup_environment(args.beach_type)

}
