use crate::manifest::Manifest;
use crate::TEMPLATE_TOKEN_REGEX;
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use regex::bytes::Regex;
use regex::{RegexSet, RegexSetBuilder};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;

pub struct TemplateToken {
    pub line_number: usize,
    pub src_path: String,
    pub dest_path: String,
    tags: Vec<String>,

}

pub struct Templates;

impl Templates {
    pub fn collect_templates(manifest: Manifest, spinner: &ProgressBar) -> Vec<String> {
        let paths = Self::load_paths_vec(manifest);
        // dbg!(&paths);
        // let path = "tmp/templates/docker-compose.yml".to_string();
        for path in paths {
            let file_buffer = BufReader::new(File::open(path).expect("cannot open file"));
            let tokens = Self::extract_template_tokens(file_buffer);
            dbg!(&tokens);
        }

        return vec![String::new()];
    }

    fn load_paths_vec(manifest: Manifest) -> Vec<String> {
        let paths_file = manifest.template_paths_file.unwrap_or_default();
        let workdir = manifest.workdir.unwrap_or_default();
        let template_file_path = format!("{}/{}", workdir, paths_file);
        let paths_as_str = fs_extra::file::read_to_string(template_file_path).unwrap();
        let paths_vec: Vec<String> = serde_json::from_str(&paths_as_str).unwrap();
        return paths_vec;
    }

    fn extract_template_tokens(file_buffer: BufReader<File>) -> Vec<(usize, String)> {
        lazy_static! {
            static ref TEMPLATE_REGEX: RegexSet = RegexSetBuilder::new(&[TEMPLATE_TOKEN_REGEX])
                .case_insensitive(true)
                .build()
                .unwrap();
        }

        let result = file_buffer
            .lines()
            .filter_map(|line| line.ok())
            .enumerate()
            .filter(|(i, line)| TEMPLATE_REGEX.is_match(line.as_str()))
            .map(|(i, line)| {
                let sanitized_line = line.trim().replace("# ", "");
                return (i, sanitized_line);
            })
            .collect::<Vec<(usize, String)>>();

        dbg!(&result);
        return result;
    }

    // pub fn new(manifest: Manifest) -> Templates {
    //     // load all paths into tuple (path, Vec<Tags>)
    //     let src_paths = Self::load_paths_vec(manifest);
    //     let processed = Self::process_src_templates(src_paths);
    //     Templates {}
    // }

    pub fn process(self: &Self, tag: String) -> Templates {
        Templates {}
    }

    // fn build_tag_map(manifest: Manifest) -> (Vec<String>, String) {
    //     return (Vec::new(), String::new());
    // }

    // pub fn run(self: &Self) {}

    // fn process_src_templates(src_paths: Vec<String>) {
    //     for src_path in src_paths {
    //         let file_str = fs_extra::file::read_to_string(src_path);
    //         if let Ok(template) = file_str {
    //             let tags = Self::extract_tags(template);
    //         }
    //     }
    // }
}

// pub(crate) fn collect_templates(manifest: Manifest, spinner: &&ProgressBar) {
//         spinner.set_message("Collecting templates.");
//         let paths_vec = Self::load_paths_vec(manifest.clone());
//         for path in paths_vec {
//             let mut loaded_file = fs_extra::file::read_to_string(&path).expect("Error loading template");
//             // replace vars
//             if let Some(replace_map) = &manifest.replace {
//                 for (k, v) in replace_map {
//                     let format_var = format!("{{{}}}", k);
//                     loaded_file = loaded_file.replace(&format_var, v);
//                 }
//             }
//             // src path
//             let src_path = Path::new(&path);

//             // dest path
//             let dest_path = Self::build_template_destination(path, &manifest);

//             fs_extra::file::write_all(dest_path, &loaded_file);
//         }
//     }

//     fn load_paths_vec(manifest: Manifest) -> Vec<String> {
//         let paths_file = manifest.template_paths_file.unwrap_or_default();
//         let workdir = manifest.workdir.unwrap_or_default();
//         let template_file_path = format!("{}/{}", workdir, paths_file);
//         let paths_as_str = fs_extra::file::read_to_string(template_file_path).unwrap();
//         let paths_vec: Vec<String> = serde_json::from_str(&paths_as_str).unwrap();
//         return paths_vec;
//     }

//     pub(crate) fn build_template_destination(path: String, manifest: &Manifest) -> PathBuf {
//         let app_name = manifest.clone().app_name.unwrap_or_default();
//         let workdir = manifest.clone().workdir.unwrap_or_default();
//         let template_dir = manifest.clone().template_dir.unwrap_or_default();
//         let to_strip = format!("{}/{}", &workdir, &template_dir);
//         let template_path = Path::new(&path);
//         let project_path = template_path
//             .strip_prefix(to_strip)
//             .expect("Error stripping prefix from path");
//         let app_path = Path::new(&app_name);
//         let dest = app_path.join(project_path);
//         return dest;
//     }
