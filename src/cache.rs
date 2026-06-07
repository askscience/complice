use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Cache {
    inner: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

struct CacheEntry {
    value: String,
    expires_at: Instant,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let cache = self.inner.lock().ok()?;
        let entry = cache.get(key)?;
        if Instant::now() < entry.expires_at {
            Some(entry.value.clone())
        } else {
            None
        }
    }

    pub fn set(&self, key: &str, value: String, ttl_secs: u64) {
        if let Ok(mut cache) = self.inner.lock() {
            cache.insert(
                key.to_string(),
                CacheEntry {
                    value,
                    expires_at: Instant::now() + Duration::from_secs(ttl_secs),
                },
            );
        }
    }
}
