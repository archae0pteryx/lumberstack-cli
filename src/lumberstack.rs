use indicatif::ProgressBar;

use crate::{ansible::core::Ansible, manifest::Manifest};

pub struct Lumberstack;

impl Lumberstack {
    pub fn start(manifest: Manifest, spinner: &ProgressBar) {
        Ansible::init_templates(manifest, &spinner);
    }
}
