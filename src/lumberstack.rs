use crate::commands::Commands;
use crate::manifest::Manifest;
use crate::templates::Templates;

use super::cli_args::CliArgs;
use super::progress::AppProgress;
use clap::Parser;

pub struct Lumberstack;

impl Lumberstack {
    pub fn run() {
        let progress_bar = AppProgress::new();
        let manifest = Manifest::new();
        let manifest_json = manifest.json;
        let only_run_these = Self::only_run();

        for build_item in manifest_json.builder.iter() {
            if only_run_these.len().eq(&0) || only_run_these.contains(&build_item.tag) {
                progress_bar.update(&build_item.feedback);

                if let Some(templates) = &build_item.templates {
                    Templates::process(templates)
                }

                if let Some(commands) = &build_item.commands {
                    Commands::process(commands);
                }
            }
        }

        progress_bar.finish("All done!");
    }

    fn only_run() -> Vec<String> {
        let args = CliArgs::parse();
        let only = args.only.unwrap_or(vec![]);
        return only;
    }
}
