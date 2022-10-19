#![allow(dead_code)]
use anyhow::{Context, Result};
use fs_extra::dir::CopyOptions;
use log::{debug, error, warn};
use std::fs;
use std::io::Write;
use std::{ffi::OsStr, fs::File, path::Path};

pub struct FileIO;

impl FileIO {
    pub fn write<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
        let f = File::create(path);
        if f.is_err() {
            error!("Unable to create file: {:?}", f);
            return Ok(());
        }
        f.unwrap().write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn copy<P: AsRef<Path>>(src: P, dest: P) -> Result<()> {
        let mut opts = fs_extra::file::CopyOptions::new();
        opts.overwrite = true;
        fs_extra::file::copy(&src, dest, &opts)
            .with_context(|| format!("Error copying file: {}", src.as_ref().to_str().unwrap()))?;
        Ok(())
    }

    // Copy entire contents of directory
    pub fn copy_dir<P: AsRef<Path>>(src: P, dest: P) {
        let mut opts = CopyOptions::new();
        opts.overwrite = true;
        opts.skip_exist = false;
        opts.copy_inside = true;
        fs_extra::dir::copy(&src, &dest, &opts).unwrap();
    }

    pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
        fs::create_dir_all(&path)
            .map_err(|_| {
                format!(
                    "Error creating directory: {}",
                    &path.as_ref().to_str().unwrap()
                )
            })
            .unwrap();
        debug!("Created directory: {}", &path.as_ref().to_str().unwrap());

        Ok(())
    }
    pub fn remove<P: AsRef<Path>>(path: P) {
        debug!("Removing {}", &path.as_ref().to_str().unwrap());
        if Path::is_dir(path.as_ref()) {
            fs_extra::dir::remove(&path).expect("Error removing directory");
        } else {
            fs_extra::file::remove(path).expect("Error removing file");
        }
    }

    pub fn read(path: &impl AsRef<Path>) -> Option<String> {
        let file_str = fs_extra::file::read_to_string(path);
        if let Ok(fs) = file_str {
            return Some(fs);
        }
        warn!(
            "[system] Could not read file to string: {}. Skipping",
            path.as_ref().to_str().unwrap()
        );
        None
    }

    pub fn is_not_contentful<P: AsRef<Path>>(path: P) -> bool {
        if Self::is_image(path.as_ref()) {
            return true;
        }
        if Path::is_dir(path.as_ref()) {
            return true;
        }
        false
    }

    pub fn is_image<P: AsRef<Path>>(path: P) -> bool {
        let mimes = vec!["jpeg", "png", "jpg", "gif"];
        let ext = Self::file_ext(&path);
        mimes.contains(&ext)
    }

    pub fn file_ext<P: AsRef<Path>>(path: &P) -> &str {
        let ext = path.as_ref().extension().and_then(OsStr::to_str);
        if let Some(e) = ext {
            return e;
        }
        ""
    }
}
