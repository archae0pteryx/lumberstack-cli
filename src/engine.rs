// use self::file_io::FileIO;

// mod file_io;

// struct AppConfig<'a> {
//   app_name: &'a str,
// }

// pub struct Engine<'a> {
//     app_config: &'a AppConfig<'a>
// }

// impl<'a> Engine<'a> {
//     pub fn new() -> Engine<'a> {
//         let config = AppConfig {
//             app_name: "test"
//         };
//         Engine { app_config: &config }
//     }

//     pub fn run(&self) {
//         // let app_config = self.app_config;
//         // let manifest = Manifest::new(app_config);
//         // let tasks = manifest.get_tasks();
//         // tasks.into_iter().for_each(|task| {
//         //     task.run_job();
//         // });
//     }
// }
