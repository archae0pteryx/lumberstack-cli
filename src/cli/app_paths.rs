use super::arguments::CliArgs;
use clap::Parser;

pub static ROOT_TEMPLATE_DIR: &str= "templates";

pub struct AppPaths;

impl AppPaths {
    pub fn root(path: Option<&str>) -> String {
        let args = CliArgs::parse();
        match args.name {
            Some(name) => {
                return match path {
                    Some(p) => format!("{}/{}/", name, p),
                    None => format!("{}", name),
                }
            }
            None => {
                return match path {
                    Some(p) => format!("myapp/{}", p),
                    None => String::from("myapp"),
                }
            }
        }
    }

    pub fn web(path: Option<&str>) -> String {
        return match path {
            Some(p) => format!("{}/{}", Self::root(Some("web")), p),
            None => format!("{}/", Self::root(Some("web"))),
        };
    }

    pub fn api(path: Option<&str>) -> String {
        return match path {
            Some(p) => format!("{}/{}", Self::root(Some("api")), p),
            None => format!("{}/", Self::root(Some("api"))),
        };
    }
}

pub struct TemplatePaths;

impl TemplatePaths {
    pub fn root(path: Option<&str>) -> String {
        return match path {
            Some(p) => format!("{}/{}", ROOT_TEMPLATE_DIR, p),
            None => format!("{}/", ROOT_TEMPLATE_DIR),
        };
    }

    pub fn web(path: Option<&str>) -> String {
        return match path {
            Some(p) => format!("{}/{}", Self::root(Some("redwood/web")), p),
            None => format!("{}/", Self::root(Some("redwood/web"))),
        };
    }

    pub fn api(path: Option<&str>) -> String {
        return match path {
            Some(p) => format!("{}/{}", Self::root(Some("redwood/api")), p),
            None => format!("{}/", Self::root(Some("redwood/api"))),
        };
    }
}
