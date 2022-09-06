use crate::commands::Commands;
use crate::manifest::Manifest;
use crate::templates::Templates;

use super::cli_args::CliArgs;
use clap::Parser;
use indicatif::ProgressBar;

pub struct Lumberstack;

impl Lumberstack {
    pub fn run(spinner: &ProgressBar) {
        let manifest = Manifest::new();
        let manifest_json = &manifest.json;
        let builder_items = &manifest_json.builder;
        let app_name = manifest.app_name;

        let only_run_these = Self::only_run();

        for build_item in builder_items.iter() {
            if only_run_these.len().eq(&0) || only_run_these.contains(&build_item.tag) {
                spinner.set_message(build_item.feedback.to_owned());

                if let Some(templates) = &build_item.templates {
                    Templates::process(templates.to_owned(), &spinner)
                }

                if let Some(commands) = &build_item.commands {
                    Commands::process(&app_name, commands.to_owned(), &spinner);
                }
                spinner.inc(1);
            }
        }
    }

    fn only_run() -> Vec<String> {
        let args = CliArgs::parse();
        let only = args.only.unwrap_or(vec![]);
        return only;
    }
}
