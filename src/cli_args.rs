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

    /// Skip system checks
    #[clap(long, action)]
    pub skip_checks: bool,

    /// Dont cleanup
    #[clap(short, long, value_parser)]
    pub artifacts: Option<String>,

    /// Load config from file
    #[clap(short, long, value_parser)]
    pub config: Option<String>,

    /// Specify a template version to use
    #[clap(long, value_parser)]
    pub template_version: Option<String>,

    /// Specific tags to run
    #[clap(short, long, value_parser, multiple(true))]
    pub tags: Option<Vec<String>>,

    /// Log ansible output to file
    #[clap(short, long, value_parser)]
    pub log: Option<String>,

    /// Skip tags
    #[clap(short, long, value_parser, multiple(true))]
    pub skip_tags: Option<Vec<String>>
}
