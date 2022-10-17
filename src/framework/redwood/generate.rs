// use crate::{
//     manifest::Manifest,
//     task_definitions::{
//         ansible::{ansible_task::RunnableAnsibleTask, yaml::command_task::CommandTask},
//         task_types::DefinedTask,
//         templates::tags::{should_task_run, TaskTag},
//     },
// };

// pub struct RedwoodGenerate;

// impl RedwoodGenerate {
//     pub fn new(tag: TaskTag, manifest: Manifest) -> Option<RunnableAnsibleTask> {
//         let app_name = manifest.app_name.to_owned().unwrap_or_default();
//         let skip_tags = &manifest.skip_tags.to_owned();

//         if !should_task_run(&tag, &manifest) {
//             return None;
//         }

//         let home_task = Self::generate_home_page(app_name);

//         None
//     }

//     fn generate_home_page(app_name: String) -> DefinedTask {
//         CommandTask::new("Generate homepage")
//             .chdir(app_name)
//             .command("foo")
//             .build()
//     }
// }
