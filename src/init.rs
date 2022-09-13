use std::{fs, process::exit};

use clap::Parser;
use log::{debug, error, info};

use crate::{
    cli_args::CliArgs, commands::Commands, default_config, logger::Logger, DEFAULT_APP_NAME,
    DEFAULT_MANIFEST_FILE, DEFAULT_TEMPLATE_DIR,
};

pub fn initialize() {
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
        debug!("Removing template dir");
        fs::remove_dir_all(DEFAULT_TEMPLATE_DIR).map_err(|e| debug!("{}", e)).expect("No dir template dir found");
        debug!("Removing docker volumes");
        Commands::exec_raw(
            DEFAULT_APP_NAME,
            "docker",
            &["compose", "down", "-v"],
            false,
        );
        debug!("Removing default app dir");
        fs::remove_dir_all(DEFAULT_APP_NAME).map_err(|e| debug!("{}", e)).expect("No app dir found");
    }
    Logger::init();
}
