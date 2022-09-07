use crate::commands::Commands;
use crate::manifest::Manifest;
use crate::templates::Templates;

use super::cli_args::CliArgs;
use clap::Parser;
use indicatif::ProgressBar;

pub struct Lumberstack;

impl Lumberstack {
    pub fn run(manifest: &Manifest, spinner: &ProgressBar) {
        let manifest_json = &manifest.json;
        let builder_items = &manifest_json.builder;
        let app_name = &manifest.app_name;

        let only_run_these = Self::only_run();

        for build_item in builder_items.iter() {
            if only_run_these.len().eq(&0) || only_run_these.contains(&build_item.tag) {
                spinner.set_message(build_item.feedback.to_owned());

                if let Some(templates) = &build_item.templates {
                    spinner.set_prefix("📄");
                    Templates::process(&app_name, templates.to_owned(), &spinner)
                }

                if let Some(commands) = &build_item.commands {
                    spinner.set_prefix("👟");
                    Commands::process(commands.to_owned(), &spinner);
                }
            }
        }
        spinner.set_prefix("✅");
        spinner.finish_with_message("Finished!")
    }

    fn only_run() -> Vec<String> {
        let args = CliArgs::parse();
        let only = args.only.unwrap_or(vec![]);
        return only;
    }
}
