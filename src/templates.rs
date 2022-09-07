use std::{fs, io::Write};

use indicatif::ProgressBar;
use log::{debug, error, info};
use fs_extra::{self, dir::CopyOptions};
use crate::manifest::TemplateItem;
use std::fs::metadata;

pub struct Templates;

impl Templates {
    pub fn process(app_name: &String, template_items: Vec<TemplateItem>, spinner: &ProgressBar) {
        for template_item in template_items.iter() {
            let feedback = template_item.feedback.to_owned();
            if let Some(feedback) = feedback {
                spinner.set_message(feedback);
            }
            let source = &template_item.source;
            let dest = &template_item.dest;


            let meta = metadata(source).expect("Cant find source to check if dir");

            if meta.is_dir() {
                Self::copy_dir(&source, &dest);
                continue;
            }

            debug!("Copying template: {} to {}", source, dest);


            let mut dest_file =
                fs::File::create(&dest).expect("💣 Error creating dest template file");

            let processed_template = Self::handle_template(&app_name, &template_item);

            let result = dest_file.write_all(processed_template.as_bytes());

            match result {
                Ok(_) => {
                    info!("Wrote template: {}", dest);
                }
                Err(_) => {
                    error!("Error writing template: {}. Continuing...", dest)
                }
            }
        }
    }

    fn handle_template(app_name: &String, template_item: &TemplateItem) -> String {
        let template_file = fs::read_to_string(&template_item.source)
        .expect(format!("Error loading template {}", &template_item.source).as_str());
        return template_file.replace("{{app_name}}", &app_name);
    }

    pub fn copy_dir(source: &String, dest: &String) {
        debug!("Copying dir {} to {}", source, dest);
        let err_message = format!("Error copying {} to {}", source, dest);
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.copy_inside = true;

        fs_extra::dir::copy(source, dest, &options).expect(&err_message);
    }
}
