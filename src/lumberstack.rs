use crate::{ansible::core::Ansible, manifest::Manifest, templates::clone::CloneTemplates};
use indicatif::ProgressBar;

pub struct Lumberstack;

impl Lumberstack {
    pub fn start(manifest: Manifest, spinner: &ProgressBar) {
        let clone_task = CloneTemplates::new(manifest.clone());
        // CloneTemplates::check_tag_and_run(manifest.clone(), spinner);
        // Ansible::init_templates(manifest.clone(), &spinner);
        // Ansible::create_redwood_app(manifest.clone(), &spinner);
        // Templates::collect_templates(manifest.clone(), &spinner);
        // Ansible::setup_docker(manifest.clone(), &spinner);
        // Ansible::generate_auth(manifest.clone(), &spinner);
    }
}
