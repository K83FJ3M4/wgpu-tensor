use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex, OnceLock};

pub(crate) struct CacheMap<K: Hash + Eq, V> {
    inner: Mutex<HashMap<K, Arc<OnceLock<Arc<V>>>>>,
}

impl<K: Hash + Eq, V> CacheMap<K, V> {
    pub(crate) fn new() -> CacheMap<K, V> {
        CacheMap {
            inner: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn get(&self, key: K, callback: impl FnOnce() -> V) -> Arc<V> {
        let mut map = match self.inner.lock() {
            Ok(map) => map,
            Err(err) => {
                let mut map = err.into_inner();
                map.clear();
                self.inner.clear_poison();
                map
            }
        };

        let value = map
            .entry(key)
            .or_insert_with(|| Arc::new(OnceLock::new()))
            .clone();
        drop(map);

        value.get_or_init(|| Arc::new(callback())).clone()
    }
}
