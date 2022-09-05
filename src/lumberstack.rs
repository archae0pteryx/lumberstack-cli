use crate::exec_command::exec_cmd;
use crate::manifest::{load_manifest, CommandStep, ManifestJson, TemplateItem};

use super::cli_args::CliArgs;
use super::progress::AppProgress;
use clap::Parser;
use handlebars::Handlebars;
use log::{error, info};
use std::io::Write;
use std::{collections::BTreeMap, fs};

pub static APP_NAME_KEY: &str = "{{app_name}}";

pub struct Lumberstack;

impl Lumberstack {
    pub fn run() {
        let progress_bar = AppProgress::new();
        let config = load_manifest();
        let app_name = Self::app_name(&config);

        let only_run_these = Self::only_runs();

        for build_item in config.builder.iter() {
            if only_run_these.len().eq(&0) || only_run_these.contains(&build_item.tag) {
                progress_bar.update(&build_item.feedback);
                if let Some(templates) = &build_item.templates {
                    Self::process_templates(&app_name, templates)
                }

                if let Some(commands) = &build_item.commands {
                    Self::process_commands(&app_name, &commands);
                }
            }
        }

        progress_bar.finish("All done!");
    }

    fn only_runs() -> Vec<String> {
        let args = CliArgs::parse();
        let only = args.only.unwrap_or(vec![]);
        return only;
    }

    // Prefer name from args
    fn app_name(config: &ManifestJson) -> String {
        let args = CliArgs::parse();
        match &args.name {
            Some(name) => {
                return name.to_owned();
            }
            None => match &config.app_name {
                Some(name) => {
                    return String::from(name);
                }
                None => return String::from("myapp"),
            },
        }
    }

    fn app_name_replace(app_name: &String, original: &String) -> String {
        Self::var_replacer(original, APP_NAME_KEY, app_name)
    }

    fn create_replace_map(
        app_name: &String,
        replace_map: Option<BTreeMap<String, String>>,
    ) -> BTreeMap<String, String> {
        let mut new_mapping: BTreeMap<String, String> = BTreeMap::new();

        if let Some(r_m) = replace_map {
            for (key, val) in r_m.iter() {
                let replaced_val = Self::app_name_replace(app_name, &val);
                new_mapping.insert(key.to_string(), replaced_val);
            }
        }

        return new_mapping;
    }

    fn handlebar_replace(app_name: &String, template_instruction: &TemplateItem) -> String {
        let source_path = Self::app_name_replace(app_name, &template_instruction.source);

        let load_src_error = format!("Error reading template: {}", source_path);

        let loaded_source_file = fs::read_to_string(source_path).expect(&load_src_error);

        let replace_map = template_instruction.replace_map.to_owned();

        let new_mapping = Self::create_replace_map(&app_name, replace_map);

        let handlebars = Handlebars::new();

        let hb_error = "💣 Error rendering template";

        let out = handlebars
            .render_template(&loaded_source_file, &new_mapping)
            .map_err(|_| error!("{}", hb_error))
            .unwrap();

        return out;
    }

    fn process_commands(app_name: &String, commands: &Vec<CommandStep>) {
        for command_step in commands.iter() {
            exec_cmd(&app_name, &command_step);
        }
    }

    fn process_templates(app_name: &String, template_instructions: &Vec<TemplateItem>) {
        for template_instruction in template_instructions.iter() {
            let dest_path = Self::app_name_replace(&app_name, &template_instruction.dest);
            let src_path = Self::app_name_replace(&app_name, &template_instruction.source);

            info!("Copying template: {} to {}", src_path, &dest_path);

            let mut dest_file =
                fs::File::create(&dest_path).expect("💣 Error creating dest template file");

            let dest_file_data = Self::handlebar_replace(app_name, template_instruction);

            dest_file
                .write_all(dest_file_data.as_bytes())
                .map_err(|_| error!("Error writing dest file: {}", &dest_path))
                .unwrap();
        }
    }

    pub fn var_replacer(original: &str, key: &str, value: &String) -> String {
        str::replace(&original, key, value)
    }
}
