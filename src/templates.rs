use std::{fs, io::Write};

use handlebars::Handlebars;
use log::{error, info};

use crate::manifest::TemplateItem;

pub struct Templates;

impl Templates {
    pub fn process(template_items: &Vec<TemplateItem>) {
        for template_item in template_items.iter() {
            let source = &template_item.source;
            let dest = &template_item.dest;

            info!("Copying template: {} to {}", &source, &dest);

            let mut dest_file =
                fs::File::create(&dest).expect("💣 Error creating dest template file");

            let processed_template = Self::handle_template(template_item);

            let result = dest_file.write_all(processed_template.as_bytes());

            match result {
                Ok(_) => {
                    info!("Wrote template: {}", dest)
                }
                Err(_) => {
                    error!("Error writing template: {}. Continuing...", dest)
                }
            }
        }
    }

    fn handle_template(template_item: &TemplateItem) -> String {
        let handlebars = Handlebars::new();
        let template_file = fs::read_to_string(&template_item.source)
            .expect(format!("Error loading template {}", &template_item.source).as_str());

        let out = handlebars
            .render_template(&template_file, &template_item.replace_map)
            .expect("Error processing template.");
        return out;
    }
}
