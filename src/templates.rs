use crate::manifest::TemplateItem;
use image;
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
        spinner.set_prefix("📄");
        for template_item in template_items.iter() {
            let feedback = template_item.feedback.to_owned();

            if let Some(feedback) = feedback {
                spinner.set_message(feedback);
            }

            let source = PathBuf::from(&template_item.source);
            let dest = PathBuf::from(&template_item.dest);

            let source_exists = Path::new(&source).exists();

            if !source_exists {
                error!("File or Folder {} does not exist", source.to_string_lossy());
                exit(exitcode::OSFILE);
            }

            let result = Self::copy_all_templates(&app_name, &source, &dest);

            if let Err(e) = result {
                error!("Error copying templates: {}", e);
                exit(exitcode::OSFILE);
            }
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

            let file_type = entry.file_type();

            // create directories
            if file_type.is_dir() {
                Self::create_dir(&to);
            } else if file_type.is_file() {
                let from = from.to_path_buf();
                Self::copy_template(&app_name, &from, to)?;
            } else {
                warn!("copy: ignored symlink {}", from.display());
            }
        }
        Ok(())
    }

    fn copy_template(
        app_name: &String,
        from: &PathBuf,
        to: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        if Self::is_image_file(from) {
            Self::write_image(from, &to);
            return Ok(());
        }

        debug!("copying template: {} to {}", from.display(), to.display());

        let to = to.clone();
        let file_str = fs::read_to_string(from);

        match &file_str {
            Ok(str) => {
                Self::write_file(&to, str, app_name);
                Ok(())
            }
            Err(_) => {
                error!("Error interpolating file [{:?}]. Skipping...", &file_str);
                Ok(())
            }
        }
    }

    fn create_dir(to: &PathBuf) {
        debug!("creating dir: {}", to.to_string_lossy());
        if let Err(e) = fs::create_dir(to) {
            match e.kind() {
                ErrorKind::AlreadyExists => {
                    debug!("already exists!");
                }
                _ => {
                    error!("Error creating destination dir! {}", e);
                    exit(1);
                }
            }
        }
    }

    fn write_file(to: &PathBuf, str: &String, app_name: &String) {
        let replaced = str.replace("{{app_name}}", &app_name);
        if let Err(e) = fs::write(to, replaced) {
            error!("error writing {} - {}", to.display(), e);
        }
    }

    fn write_image(from: &PathBuf, to: &PathBuf) {
        debug!("writing image {} to {}", from.display(), to.display());
        match image::open(from) {
            Ok(img) => {
                if let Err(e) = img.save(to) {
                    error!("Error writing image: {} - {}", to.display(), e);
                }
            }
            Err(e) => {
                error!("Error opening image: {} - {}", from.display(), e);
                exit(exitcode::OSFILE);
            }
        }
    }

    fn is_image_file(from: &PathBuf) -> bool {
        let has_png = from.to_string_lossy().contains(".png");
        let has_jpg = from.to_string_lossy().contains(".jpg");
        let has_jpeg = from.to_string_lossy().contains(".jpeg");
        let is_image = has_png | has_jpg | has_jpeg;
        debug!("{} is an image: {}", from.display(), is_image);
        return is_image;
    }
}
