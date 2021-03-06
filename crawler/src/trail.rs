use crate::utils;
use std::collections::BTreeSet;

pub struct Trail {
    store: BTreeSet<String>,
}

impl Trail {
    pub fn new() -> Self {
        Self {
            store: BTreeSet::new(),
        }
    }

    pub fn set(&mut self, key: &str) {
        key = utils::no_trailing_slash(key);

        self.store.insert(key.to_string());
    }

    pub fn has(&self, key: &str) -> bool {
        key = utils::no_trailing_slash(key);

        self.store.contains(key)
    }
}
