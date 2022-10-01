use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// Opinionated typescript project generator with a RedwoodJS core
#[derive(Parser, Debug, Clone)]
#[clap(version, about, long_about = None)]
pub struct CliArgs {
    /// Project name and path.
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

    /// Cleanup all cli created files/folders
    #[clap(long, action)]
    pub clean: bool,

    /// Load config from file
    #[clap(short, long, value_parser)]
    pub config: Option<String>,

    // Specify a template version to use
    #[clap(short, long, value_parser)]
    pub template_version: Option<String>,
}
