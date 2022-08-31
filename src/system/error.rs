use std::{fmt, io};

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\u{0020}💣 App Error: {}", self.message)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            message: error.to_string(),
        }
    }
}
impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError {
            message: error.to_string(),
        }
    }
}
