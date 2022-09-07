use crate::manifest::TemplateItem;
use indicatif::ProgressBar;
use log::{debug, error, warn};
use std::error::Error;
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::exit,
};
use walkdir::WalkDir;

pub struct Templates;

impl Templates {
    pub fn process(app_name: &String, template_items: Vec<TemplateItem>, spinner: &ProgressBar) {
        for template_item in template_items.iter() {
            let feedback = template_item.feedback.to_owned();

            if let Some(feedback) = feedback {
                spinner.set_message(feedback);
            }

            let source = PathBuf::from(&template_item.source);
            let dest = PathBuf::from(&template_item.dest);

            let source_exists = Path::new(&source).exists();

            if !source_exists {
                error!("File / Folder {} does not exist", source.to_string_lossy());
                exit(exitcode::OSFILE);
            }

            Self::copy_all_templates(&app_name, &source, &dest)
                .expect("Error copying all templates");
        }
    }

    fn copy_all_templates(
        app_name: &String,
        in_dir: &PathBuf,
        out_dir: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        for entry in WalkDir::new(&in_dir) {
            let entry = entry?;

            let from = entry.path();
            let to = out_dir.join(from.strip_prefix(&in_dir)?);

            debug!("Copying {} to {}", from.display(), to.display());

            // create directories
            if entry.file_type().is_dir() {
                if let Err(e) = fs::create_dir(to) {
                    match e.kind() {
                        ErrorKind::AlreadyExists => {}
                        _ => return Err(e.into()),
                    }
                }
            } else if entry.file_type().is_file() {
                let from = from.to_path_buf();
                Self::copy_template(&app_name, &from, &to)?;
            } else {
                warn!("copy: ignored symlink {}", from.display());
            }
        }
        Ok(())
    }

    fn copy_template(
        app_name: &String,
        from: &PathBuf,
        to: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let mut to = to.clone();
        let file_str = fs::read_to_string(from);

        match &file_str {
            Ok(str) => {
                if !to
                    .extension()
                    .unwrap()
                    .to_string_lossy()
                    .eq(&String::from("template"))
                {
                    warn!(
                        "Found non template: {} in template dir. Skipping.",
                        to.to_string_lossy()
                    );
                    return Ok(());
                }

                let replaced = str.replace("{{app_name}}", &app_name);

                to.set_extension("".to_string());
                fs::write(to, replaced)?;

                Ok(())
            }
            Err(_) => {
                warn!("Error interpolating file [{:?}]. Skipping...", &file_str);
                Ok(())
            }
        }
    }
}
