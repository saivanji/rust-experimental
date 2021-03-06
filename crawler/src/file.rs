use crate::Location;
use anyhow::{anyhow, Result};
use std::fs;
use std::io::Write;
use std::str;

pub struct File<'a> {
    blob: Vec<u8>,
    loc: &'a Location,
}

impl<'a> File<'a> {
    pub fn from(blob: Vec<u8>, loc: &'a Location) -> Self {
        Self { blob, loc }
    }

    pub fn persist(&self) -> Result<()> {
        let path = &self.loc.path;
        let display = path.display();

        let file_create_err = Err(anyhow!("Can not create file {}", display));
        let file_write_err = Err(anyhow!("Can not write to {} file", display));
        let dir_create_err = Err(anyhow!("Can not create {} directory", display));

        let prefix = path.parent().unwrap_or(path);
        std::fs::create_dir_all(prefix).or(dir_create_err)?;

        fs::File::create(path)
            .or(file_create_err)?
            .write(&self.blob)
            .or(file_write_err)?;

        Ok(())
    }

    pub fn text(&self) -> Option<&str> {
        str::from_utf8(&self.blob).ok()
    }
}
