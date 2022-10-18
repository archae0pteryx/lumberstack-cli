use std::{ffi::OsStr, path::Path, fs::File};
use std::io::Write;
use anyhow::{Context, Result};
use log::{warn, error};

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

    pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
        fs_extra::dir::create_all(&path, false).with_context(|| {
            format!(
                "Failed to create directories for path: {}",
                &path.as_ref().to_str().unwrap()
            )
        })?;
        // let mut opts = CopyOptions::new();
        //     opts.overwrite = true;
        //     opts.skip_exist = false;
        //     opts.copy_inside = true;
        //     fs_extra::dir::copy(&src, &dest, &opts).unwrap();
        Ok(())
    }

    pub fn read(path: &impl AsRef<Path>) -> Option<String> {
        let file_str = fs_extra::file::read_to_string(&path);
        if let Ok(fs) = file_str {
            return Some(fs);
        }
        warn!(
            "[system] Could not read file to string: {}. Skipping",
            path.as_ref().to_str().unwrap()
        );
        return None;
    }

    pub fn is_image<P: AsRef<Path>>(path: P) -> bool {
        let mimes = vec!["jpeg", "png", "jpg", "gif"];
        let ext = Self::file_ext(&path);
        return mimes.contains(&ext);
    }

    pub fn file_ext<P: AsRef<Path>>(path: &P) -> &str {
        let ext = path.as_ref().extension().and_then(OsStr::to_str);
        if let Some(e) = ext {
            return e;
        }
        return "";
    }
}
