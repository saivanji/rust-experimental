use std::collections::HashMap;
use std::path::PathBuf;

/// Stores key/value pairs.
///
/// The data stored is kept in memory and not persisted on the disk.
///
/// Example:
///
/// ```rust
/// use kvs::KvStore;
///
/// let mut store = KvStore::new();
///
/// store.set("foo".to_owned(), "bar".to_owned());
/// let value = store.get("foo".to_owned());
///
/// assert_eq!(value, Some("bar".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        let store = HashMap::new();

        KvStore { store }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        panic!();
    }

    /// Retrieves value from a store.
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Sets value to store for a given key.
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Removes value from the store.
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
