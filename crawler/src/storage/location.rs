use crate::utils;
use std::path::{Path, PathBuf};

pub struct Location {
    pub path: PathBuf,
}

impl Location {
    pub fn new(input: &str) -> Self {
        let input = utils::no_leading_slash(input);

        Self {
            path: Path::new("./out").join(input),
        }
    }

    pub fn concat(&self, path: &str) -> Self {
        let path = utils::no_leading_slash(path);

        Self {
            path: self.path.join(path),
        }
    }
}
