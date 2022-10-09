// #![allow(unused)]

use super::playbook::{yaml::task_type::TaskType, self, create::Playbook};

pub fn execute_playbook(playbook: Playbook) {
  
}

//     pub(crate) fn create_redwood_app(manifest: Manifest) -> TaskType {
//         let app_name = &manifest.app_name.unwrap_or_default();
//         let workdir = &manifest.workdir.unwrap_or_default();
//         let command = format!(
//                 "yarn create redwood-app {} --typescript --overwrite > {}/create.stdout.log 2> {}/create.stderr.log",
//                 app_name,
//                 workdir,
//                 workdir
//             );
//         CommandTask::new("Create redwood app")
//             .command(command.as_str())
//             .creates(app_name.as_str())
//             .register("create_command")
//             .build()
//     }

//     pub(crate) fn setup_db_auth(manifest: Manifest) -> TaskType {
//         let app_name = manifest.app_name.unwrap_or_default();
//         CommandTask::new("Setup redwood auth")
//             .command("yarn rw setup auth dbAuth --force")
//             .chdir(app_name.as_str())
//             .build()
//     }

//     pub(crate) fn generate_auth(manifest: Manifest) -> TaskType {
//         let app_name = manifest.app_name.unwrap_or_default();
//         CommandTask::new("Generating auth")
//             .command("echo fooabar")
//             .chdir(app_name.as_str())
//             .build()
//     }

//     pub(crate) fn copy_compose(manifest: Manifest) -> TaskType {
//         let app_name = manifest.app_name.unwrap_or_default();
//         CommandTask::new("").build()
//     }
// }
