use crate::{
    ansible::playbook::create::Playbook,
    manifest::Manifest,
    tags::{should_task_run, TaskTag},
};

pub struct DockerCompose;

impl DockerCompose {
    pub fn new(tag: TaskTag, manifest: Manifest) -> Option<Playbook> {
        let tags = &manifest.tags.to_owned();

        if !should_task_run(&tag, tags) {
            return None;
        }

        let base_playbook = Playbook::new("Docker Compose");

        Some(base_playbook)
    }

}
