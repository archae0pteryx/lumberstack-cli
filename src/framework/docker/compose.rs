// use crate::{
//     manifest::Manifest, task_definitions::{ansible::ansible_task::RunnableAnsibleTask, templates::tags::{should_task_run, TaskTag}},
// };

// pub struct DockerCompose;

// impl DockerCompose {
//     pub fn new(tag: TaskTag, manifest: Manifest) -> Option<RunnableAnsibleTask> {
//         let tags = &manifest.tags.to_owned();
//         let skip_tags = &manifest.skip_tags.to_owned();
//         if !should_task_run(&tag, &manifest) {
//             return None;
//         }

//         let base_playbook = RunnableAnsibleTask::new("Docker Compose");

//         Some(base_playbook)
//     }

// }
