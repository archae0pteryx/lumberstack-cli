use clap::{Args, Parser, Subcommand};
use strum::EnumIter;

/// Opinionated typescript project generator with a RedwoodJS core
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct CliArgs {
    /// Project name and path
    #[clap(value_parser)]
    pub name: Option<String>,

    /// Remove project directory first
    #[clap(long, action)]
    pub clean: bool,


    /// Skip creating new redwood app
    #[clap(short, long, action)]
    pub not_redwood: bool,

    /// Verbose logging
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Run single generators
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run single generators
    Only(Only),
}

#[derive(Args, Debug)]
struct Only {
    /// Create new redwood app
    #[clap(long, action)]
    pub redwood: bool,

    /// Run system checks
    #[clap(long, action)]
    pub system: bool,

    /// Run prisma setup
    #[clap(long, action)]
    pub prisma: bool,

    /// Run playwright setup
    #[clap(long, action)]
    pub playwright: bool,

    /// Setup Heroku deployment
    #[clap(long, action)]
    pub heroku: bool,

    /// Copy base templates
    #[clap(long, action)]
    pub templates: bool,

    /// Run web / api unit tests
    #[clap(long, action)]
    pub unit_tests: bool,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum CliOptions {
    Redwood,
    System,
    Prisma,
    Playwright,
    Heroku,
    Templates,
    UnitTests,
}

pub fn is_enabled(command: &CliOptions) -> bool {
    let args = CliArgs::parse();
    return match &args.command {
        Some(Commands::Only(c)) => match command {
            CliOptions::Redwood => c.redwood,
            CliOptions::System => c.system,
            CliOptions::Prisma => c.prisma,
            CliOptions::Playwright => c.playwright,
            CliOptions::Heroku => c.heroku,
            CliOptions::Templates => c.templates,
            CliOptions::UnitTests => c.unit_tests
        },
        None => false,
    };
}

pub fn has_only_enabled() -> bool {
    let args = CliArgs::parse();
    return args.command.is_some();
}
