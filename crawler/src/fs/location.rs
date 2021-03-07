use crate::utils;
use std::path::{Path, PathBuf};

pub struct Location {
    pub path: PathBuf,
}

impl Location {
    pub fn new(path: &str) -> Self {
        Self {
            path: Path::new("./out").join(utils::no_leading_slash(path)),
        }
    }

    pub fn concat(&self, path: &str) -> Self {
        Self {
            path: self.path.join(utils::no_leading_slash(path)),
        }
    }
}
