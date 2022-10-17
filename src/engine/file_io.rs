use std::collections::HashMap;

use anyhow::{Result, Error};

pub trait FileIO {
    fn load_file(path: &str) -> Result<String, Error> {
        let contents = fs_extra::file::read_to_string(path)?;
        Ok(contents)
    }

    fn write_file(path: &str, contents: &str) -> Result<(), Error> {
        fs_extra::file::write_all(path, contents)?;
        Ok(())
    }

    fn copy_file(src: &str, dest: &str) -> Result<(), Error> {
        fs_extra::file::copy(src, dest, &fs_extra::file::CopyOptions::new())?;
        Ok(())
    }
}
