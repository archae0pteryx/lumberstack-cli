use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// Opinionated typescript project generator with a RedwoodJS core
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct CliArgs {
    /// Project name and path. Overrides manifest value (if present)
    #[clap(value_parser)]
    pub name: Option<String>,

    /// Verbose logging (-v = warn, -vv = info)
    #[clap(flatten)]
    pub verbose: Verbosity,

    /// Run tag(s) (comma separated)
    #[clap(short, long)]
    pub only: Option<Vec<String>>,

    /// Disable system checks
    #[clap(short, long, action)]
    pub disable_checks: bool,

    /// Load config from file
    #[clap(short, long, value_parser)]
    pub config: Option<String>
}
