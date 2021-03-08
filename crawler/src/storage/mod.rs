use crate::Location;
use anyhow::{anyhow, Result};
use std::fs;

pub mod file;
pub mod location;

pub fn cleanup(workdir: &Location) -> Result<()> {
    if workdir.path.exists() {
        fs::remove_dir_all(&workdir.path).or(Err(anyhow!("Can not remove website directory")))?;
    }

    Ok(())
}
