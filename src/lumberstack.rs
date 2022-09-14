use crate::commands::Commands;
use crate::manifest::{CommandItem, Manifest, TemplateItem};
use crate::templates::Templates;

use super::cli_args::CliArgs;
use clap::Parser;
use indicatif::ProgressBar;
use serde_json::{Error, Map, Value};

pub struct Lumberstack;

impl Lumberstack {
    pub fn run(manifest: &Manifest, spinner: &ProgressBar) {
        let manifest_json = &manifest.json;
        let builder_items = &manifest_json.builder;
        let app_name = &manifest.app_name;

        let only_run_these = Self::only_run();

        let items: Vec<Map<String, Value>> = builder_items
            .iter()
            .filter(|item| {
                return only_run_these.contains(&item.tag) || only_run_these.is_empty();
            })
            .map(serde_json::to_value)
            .map(Result::unwrap)
            .map(|r| r.as_object().cloned())
            .map(Option::unwrap)
            .collect();

        for item in items {
            Self::process_item(app_name, item, spinner);
        }

        spinner.set_prefix("✅");
        spinner.finish_with_message("Finished!")
    }

    fn only_run() -> Vec<String> {
        let args = CliArgs::parse();
        let only = args.only.unwrap_or(vec![]);
        return only;
    }

    fn process_item(app_name: &String, item: Map<String, Value>, spinner: &ProgressBar) {
        // Process items in order
        for (k, v) in item.iter() {
            if k.contains("commands") {
                let commands: Result<Vec<CommandItem>, Error> =
                    serde_json::from_value(v.to_owned());
                if let Ok(c) = commands {
                    Commands::process(c, spinner);
                }
            }

            if k.contains("templates") {
                let templates: Result<Vec<TemplateItem>, Error> =
                    serde_json::from_value(v.to_owned());
                if let Ok(t) = templates {
                    Templates::process(app_name, t, spinner);
                }
            }

            if k.contains("feedback") {
                let v: Result<String, Error> = serde_json::from_value(v.to_owned());
                spinner.set_message(v.unwrap_or(String::from("UNKNOWN")));
            }
        }
    }
}
