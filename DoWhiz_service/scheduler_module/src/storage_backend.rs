use std::env;

use tracing::warn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBackend {
    Sqlite,
    Dual,
    Mongo,
}

impl StorageBackend {
    pub fn from_env() -> Self {
        let _ = crate::env_alias::apply_deploy_target_overrides();
        let raw = env::var("STORAGE_BACKEND").unwrap_or_else(|_| "sqlite".to_string());
        match raw.trim().to_ascii_lowercase().as_str() {
            "sqlite" => Self::Sqlite,
            "dual" => Self::Dual,
            "mongo" | "mongodb" => Self::Mongo,
            other => {
                warn!(
                    "unknown STORAGE_BACKEND='{}'; falling back to sqlite",
                    other
                );
                Self::Sqlite
            }
        }
    }

    pub fn uses_mongo(self) -> bool {
        matches!(self, Self::Dual | Self::Mongo)
    }

    pub fn uses_sqlite(self) -> bool {
        matches!(self, Self::Dual | Self::Sqlite)
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
    fn storage_backend_defaults_to_sqlite() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _backend = EnvGuard::set("STORAGE_BACKEND", "");
        assert_eq!(StorageBackend::from_env(), StorageBackend::Sqlite);
    }

    #[test]
    fn storage_backend_parses_mongo_and_dual() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _mongo = EnvGuard::set("STORAGE_BACKEND", "mongo");
        assert_eq!(StorageBackend::from_env(), StorageBackend::Mongo);
        drop(_mongo);
        let _dual = EnvGuard::set("STORAGE_BACKEND", "dual");
        assert_eq!(StorageBackend::from_env(), StorageBackend::Dual);
    }
}
