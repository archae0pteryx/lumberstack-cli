use std::{fs, path::Path, process::exit};

use clap::Parser;
use log::{debug, error, info};

use crate::{
    cli_args::CliArgs, commands::Commands, default_config, logger::Logger, DEFAULT_APP_NAME,
    DEFAULT_MANIFEST_FILE, DEFAULT_TEMPLATE_DIR,
};

pub fn initialize() {
    Logger::init();
    let args = CliArgs::parse();
    if args.init {
        if let Err(_) = fs::write(
            DEFAULT_MANIFEST_FILE,
            default_config::generate_default_config(),
        ) {
            error!("Error creating init config");
            exit(exitcode::IOERR)
        }
        info!("Default config written!");
        exit(exitcode::OK);
    }
    if args.clean {
        info!("Cleaning...");
        if Path::new(DEFAULT_TEMPLATE_DIR).exists() {
            debug!("Removing template dir");
            fs::remove_dir_all(DEFAULT_TEMPLATE_DIR).expect("Dir remove err");
        }
        debug!("Removing docker volumes");
        Commands::exec_raw(
            DEFAULT_APP_NAME,
            "docker",
            &["compose", "down", "-v"],
            false,
        );
        if Path::new(DEFAULT_APP_NAME).exists() {
            debug!("Removing default app dir");
            fs::remove_dir_all(DEFAULT_APP_NAME).expect("Dir remove err");
        }
    }
}
