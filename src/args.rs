use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SandboxArgs {
    /// Create a New Beach
    #[clap(short = 'n', long = "new", default_value = "")]
    pub beach_type: String,

    /// Search for Environment
    #[clap(short = 'S', long, default_value = "")]
    pub search: String,

    /// Search for Environment
    #[clap(short = 'I', long, default_value = "")]
    pub install: String,
}

