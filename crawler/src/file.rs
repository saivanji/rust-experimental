use anyhow::Result;
use std::path::Path;

pub struct File {
    blob: Vec<u8>,
    path: Path,
}

impl File {
    pub fn from(bytes: Vec<u8>, path: &str) -> Self {
        Self {
            blob: bytes,
            path: Path::new(path),
        }
    }

    pub fn persist() -> Result<()> {}

    pub fn markup() -> Option<Markup> {}
}
