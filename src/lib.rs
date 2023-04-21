mod editor;
mod config;
mod args;
mod environment;

use args::SandboxArgs;
use clap::Parser;
use environment::go_to_environment;

pub fn run() { 
    let args = SandboxArgs::parse();
    go_to_environment(args.beach_type)
}
