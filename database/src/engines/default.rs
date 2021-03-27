use crate::Engine;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{to_writer, Deserializer};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

/// Stores key/value pairs.
///
/// The data stored is kept in memory and not persisted on the disk.
pub struct DefaultEngine {
    writer: BufWriter<File>,
    data: HashMap<String, String>,
}

impl DefaultEngine {
    pub fn open(path: PathBuf) -> Result<Self> {
        let mut reader = BufReader::new(open_log_file(&path)?);
        let writer = BufWriter::new(open_log_file(&path)?);
        let data = load_data(&mut reader)?;

        Ok(Self { writer, data })
    }
}

// TODO: do not keep k/v in memory
// TODO: add compaction
impl Engine for DefaultEngine {
    /// Opens DefaultEngine at a given path
    /// Retrieves value from a store.
    fn get(&self, key: String) -> Result<Option<String>> {
        let x = self.data.get(&key).map(|s| String::from(s));

        Ok(x)
    }

    /// Sets value to store for a given key.
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key.to_owned(),
            value: value.to_owned(),
        };

        self.data.insert(key, value);
        to_writer(&mut self.writer, &cmd)?;

        Ok(())
    }

    /// Removes value from the store.
    fn remove(&mut self, key: String) -> Result<()> {
        let cmd = Command::Remove {
            key: key.to_owned(),
        };

        self.data.remove(&key);
        to_writer(&mut self.writer, &cmd)?;

        Ok(())
    }
}

fn open_log_file(path: &Path) -> Result<File> {
    fs::create_dir_all(&path)?;

    let log_path = path.join("db.log");

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(log_path)?;

    Ok(file)
}

fn load_data(input: &mut BufReader<File>) -> Result<HashMap<String, String>> {
    let mut stream = Deserializer::from_reader(input).into_iter::<Command>();
    let mut data = HashMap::new();

    while let Some(cmd) = stream.next() {
        match cmd? {
            Command::Set { key, value } => {
                data.insert(key, value);
            }
            Command::Remove { key } => {
                data.remove(&key);
            }
        }
    }

    Ok(data)
}

/// Struct representing a command.
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
