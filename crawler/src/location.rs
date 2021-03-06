use std::path::{Path, PathBuf};

pub struct Location {
    pub path: PathBuf,
}

impl Location {
    pub fn new(path: &str) -> Self {
        path = no_leading_slash(path);

        Self {
            path: Path::new("./out").join(path),
        }
    }

    pub fn concat(&self, path: &str) -> Self {
        path = no_leading_slash(path);

        Self {
            path: self.path.join(path),
        }
    }
}

fn no_leading_slash(input: &str) -> &str {
    if input == "/" || input.chars().count() == 0 || &input[0..1] != "/" {
        return input;
    }

    no_leading_slash(input.strip_prefix("/").unwrap_or(input))
}
