use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

pub(crate) struct CacheMap<K: Hash + Eq, V: Clone> {
    inner: Mutex<HashMap<K, V>>
}

impl<K: Hash + Eq, V: Clone> CacheMap<K, V> {
    pub(crate) fn new() -> CacheMap<K, V> {
        CacheMap {
            inner: Mutex::new(HashMap::new())
        }
    }

    pub(crate) fn get(&self, key: K, callback: impl FnOnce() -> V) -> V {
        let mut value = match self.inner.lock()
            .map(|map| map.get(&key).cloned()) {
            Ok(value) => value,
            Err(err) => {
                let mut map = err.into_inner();
                *map = HashMap::new();
                self.inner.clear_poison();
                None
            }
        };

        let value = value.get_or_insert_with(callback)
            .clone();

        if let Ok(mut map) = self.inner.lock() {
            map.insert(key, value.clone());
        }

        value
    }
}