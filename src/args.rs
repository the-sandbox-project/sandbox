use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct SandboxArgs {
    /// Create a New Environment
    #[clap(short, long, default_value = "")]
    pub new: String,

    /// Create a New Environment
    #[clap(short = 'U', long, default_value = "")]
    pub uninstall: String,

    /// Search for Environment
    #[clap(short = 'S', long, default_value = "")]
    pub search: String,

    /// Search for Environment
    #[clap(short = 'I', long, default_value = "")]
    pub install: String,

    /// Clear the Install Cache
    #[clap(short = 'C', long)]
    pub clearcache: bool,
}

