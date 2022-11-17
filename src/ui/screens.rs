pub(crate) mod common;
pub(crate) mod home;
pub(crate) mod progress;
pub(crate) mod new_project;
pub(crate) mod tag_select;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Screen {
    Home,
    TagSelect,
    NewProject,
    Progress,
    None,
    Quit,
}
