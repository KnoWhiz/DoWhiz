use std::env;

use tracing::warn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBackend {
    Mongo,
}

impl StorageBackend {
    pub fn from_env() -> Self {
        let raw = env::var("STORAGE_BACKEND").unwrap_or_else(|_| "mongo".to_string());
        match raw.trim().to_ascii_lowercase().as_str() {
            "mongo" | "mongodb" => Self::Mongo,
            other => {
                warn!(
                    "unsupported STORAGE_BACKEND='{}'; forcing mongo backend",
                    other
                );
                Self::Mongo
            }
        }
    }

    pub fn uses_mongo(self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;

    use super::StorageBackend;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = env::var(key).ok();
            env::set_var(key, value);
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                env::set_var(self.key, previous);
            } else {
                env::remove_var(self.key);
            }
        }
    }

    #[test]
    fn storage_backend_defaults_to_mongo() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _backend = EnvGuard::set("STORAGE_BACKEND", "");
        assert_eq!(StorageBackend::from_env(), StorageBackend::Mongo);
    }

    #[test]
    fn storage_backend_accepts_legacy_values_as_mongo() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _mongo = EnvGuard::set("STORAGE_BACKEND", "mongo");
        assert_eq!(StorageBackend::from_env(), StorageBackend::Mongo);
        drop(_mongo);
        let _legacy = EnvGuard::set("STORAGE_BACKEND", "legacy");
        assert_eq!(StorageBackend::from_env(), StorageBackend::Mongo);
    }
}
