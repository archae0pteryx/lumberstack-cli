use crate::manifest::Manifest;


enum Tag {
  Init,
  Create,
  Docker
}

pub struct Templates;

/// Templates::new(manifest).process(tag)

impl Templates {
    pub fn new(manifest: Manifest) -> Templates {
        // load all paths into tuple (path, Vec<Tags>)
        let src_paths = Self::load_paths_vec(manifest);
        let processed = Self::process_src_templates(src_paths);
        Templates {}
    }

    pub fn process(tag: String) -> Templates {
        Templates {}
    }


    fn build_tag_map(manifest: Manifest) -> (Vec<String>, String) {
      return (Vec::new(), String::new())
    }

    fn load_paths_vec(manifest: Manifest) -> Vec<String> {
        let paths_file = manifest.template_paths_file.unwrap_or_default();
        let workdir = manifest.workdir.unwrap_or_default();
        let template_file_path = format!("{}/{}", workdir, paths_file);
        let paths_as_str = fs_extra::file::read_to_string(template_file_path).unwrap();
        let paths_vec: Vec<String> = serde_json::from_str(&paths_as_str).unwrap();
        return paths_vec;
    }

    pub fn run(self: &Self) {

    }

    fn process_src_templates(src_paths: Vec<String>) {
        for src_path in src_paths {
          let file_str = fs_extra::file::read_to_string(src_path);
          if let Ok(template) = file_str {
            let tags = Self::extract_tags(template);
          }
        }
    }

    fn extract_tags(template: String) -> Vec<String> {
        
        todo!()
    }
}
