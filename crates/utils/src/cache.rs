use std::{collections::HashMap, hash::Hash};

/// Simple wrapper around a HashMap to cache values.
pub struct Cache<T: Eq + Hash, U>(HashMap<T, U>);

impl<T: Eq + Hash, U> Cache<T, U> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &T) -> Option<&U> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut U> {
        self.0.get_mut(key)
    }

    pub fn get_or_insert_with(&mut self, key: T, f: impl FnOnce() -> U) -> &U {
        self.0.entry(key).or_insert_with(f)
    }

    pub fn insert(&mut self, key: T, value: U) -> Option<U> {
        self.0.insert(key, value)
    }
}

impl<T: Eq + Hash, U: Clone> Cache<T, U> {
    /// Update the cache with the key and value, returning the value.
    /// Value is cloned upon insertion.
    pub fn update(&mut self, key: T, value: U) -> U {
        self.0.insert(key, value.clone());
        value
    }
}

impl<T: Eq + Hash, U> Default for Cache<T, U> {
    fn default() -> Self {
        Self::new()
    }
}
