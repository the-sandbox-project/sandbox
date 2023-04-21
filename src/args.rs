use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct SandboxArgs {
    /// Create a New Beach
    #[clap(short = 'n', long = "new")]
    pub beach_type: String,
}

