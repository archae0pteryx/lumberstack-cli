use std::fmt::Display;

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
    pub skip_tags: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct ParsedArgs {
    pub name: Option<String>,
    pub verbose: Verbosity,
    pub skip_checks: bool,
    pub artifacts: Option<String>,
    pub config: Option<String>,
    pub template_version: Option<String>,
    pub tags: Option<Vec<String>>,
    pub log: Option<String>,
    pub skip_tags: Option<Vec<String>>,
}

impl Default for ParsedArgs {
    fn default() -> Self {
        Self {
            name: None,
            verbose: Verbosity::new(0, 0),
            skip_checks: false,
            artifacts: None,
            config: None,
            template_version: None,
            tags: None,
            log: None,
            skip_tags: None,
        }
    }
}

impl ParsedArgs {
    pub fn new() -> Self {
        let args = CliArgs::parse();
        Self {
            name: args.name,
            verbose: args.verbose,
            skip_checks: args.skip_checks,
            artifacts: args.artifacts,
            config: args.config,
            template_version: args.template_version,
            tags: args.tags,
            log: args.log,
            skip_tags: args.skip_tags,
        }
    }
}
