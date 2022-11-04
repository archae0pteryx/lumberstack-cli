use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// Generator for Opinionated RedwoodJS Projects
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

    /// Remove previous artifacts
    #[clap(long, action)]
    pub clean: bool,

    /// Run interactively
    #[clap(short, long, action)]
    pub interactive: bool,

    /// Specifiy a config from file
    #[clap(short, long, value_parser)]
    pub config: Option<String>,

    /// Specify a template version
    #[clap(long, value_parser)]
    pub template_version: Option<String>,

    /// Specific tags to run
    #[clap(short, long, value_parser, multiple(true))]
    pub tags: Option<Vec<String>>,

    /// Log output to file
    #[clap(short, long, value_parser)]
    pub log_file: Option<String>,

    /// Tasks to skip
    #[clap(short, long, value_parser, multiple(true))]
    pub skip_tags: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct ParsedArgs {
    pub name: Option<String>,
    pub verbose: Verbosity,
    pub skip_checks: bool,
    pub config: Option<String>,
    pub template_version: Option<String>,
    pub tags: Option<Vec<String>>,
    pub log_file: Option<String>,
    pub skip_tags: Option<Vec<String>>,
    pub clean: bool,
    pub interactive: bool,
}

impl Default for ParsedArgs {
    fn default() -> Self {
        Self {
            name: None,
            verbose: Verbosity::new(0, 0),
            skip_checks: false,
            config: None,
            template_version: None,
            tags: None,
            log_file: None,
            skip_tags: None,
            clean: false,
            interactive: true
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
            config: args.config,
            template_version: args.template_version,
            tags: args.tags,
            log_file: args.log_file,
            skip_tags: args.skip_tags,
            clean: args.clean,
            interactive: args.interactive,
        }
    }
}
