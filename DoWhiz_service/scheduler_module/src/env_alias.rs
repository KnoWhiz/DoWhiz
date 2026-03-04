use std::env;

fn read_trimmed_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn var_with_scale_oliver(key: &str) -> Option<String> {
    let prefixed = format!("SCALE_OLIVER_{key}");
    let deploy_target = env::var("DEPLOY_TARGET")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "production".to_string());
    if deploy_target == "production" {
        read_trimmed_env(key).or_else(|| read_trimmed_env(&prefixed))
    } else {
        read_trimmed_env(&prefixed).or_else(|| read_trimmed_env(key))
    }
}

pub fn bool_with_scale_oliver(key: &str, default_value: bool) -> bool {
    var_with_scale_oliver(key)
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default_value)
}

const SCALE_ALIAS_SYNC_KEYS: &[&str] = &[
    "INGESTION_QUEUE_BACKEND",
    "SERVICE_BUS_CONNECTION_STRING",
    "SERVICE_BUS_QUEUE_NAME",
    "SERVICE_BUS_TEST_QUEUE_NAME",
    "SERVICE_BUS_NAMESPACE",
    "SERVICE_BUS_POLICY_NAME",
    "SERVICE_BUS_POLICY_KEY",
    "RAW_PAYLOAD_STORAGE_BACKEND",
    "RAW_PAYLOAD_PATH_PREFIX",
    "AZURE_STORAGE_ACCOUNT",
    "AZURE_STORAGE_CONTAINER_INGEST",
    "AZURE_STORAGE_SAS_TOKEN",
    "AZURE_STORAGE_CONTAINER_SAS_URL",
    "AZURE_STORAGE_CONNECTION_STRING_INGEST",
];

pub fn apply_deploy_target_overrides() -> Result<(), String> {
    let raw_target = env::var("DEPLOY_TARGET").unwrap_or_else(|_| "production".to_string());
    let normalized_target = raw_target.trim().to_ascii_lowercase();
    match normalized_target.as_str() {
        "production" | "staging" => {}
        _ => {
            return Err(format!(
                "Invalid DEPLOY_TARGET='{}'. Expected 'production' or 'staging'.",
                raw_target
            ));
        }
    }
    env::set_var("DEPLOY_TARGET", &normalized_target);

    if normalized_target != "staging" {
        return Ok(());
    }

    let staging_vars: Vec<(String, String)> = env::vars()
        .filter(|(key, _)| key.starts_with("STAGING_"))
        .collect();
    for (key, value) in staging_vars {
        let base_key = key.trim_start_matches("STAGING_");
        if !base_key.is_empty() {
            env::set_var(base_key, value);
        }
    }

    for key in SCALE_ALIAS_SYNC_KEYS {
        if let Some(value) = read_trimmed_env(key) {
            env::set_var(format!("SCALE_OLIVER_{key}"), value);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::var_with_scale_oliver;
    use std::env;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn var_with_scale_oliver_prefers_base_in_production() {
        let _guard = env_lock().lock().expect("env lock");
        env::set_var("DEPLOY_TARGET", "production");
        env::set_var("TEST_ALIAS_KEY", "base-value");
        env::set_var("SCALE_OLIVER_TEST_ALIAS_KEY", "scale-value");
        let value = var_with_scale_oliver("TEST_ALIAS_KEY").expect("value");
        assert_eq!(value, "base-value");
        env::remove_var("TEST_ALIAS_KEY");
        env::remove_var("SCALE_OLIVER_TEST_ALIAS_KEY");
        env::remove_var("DEPLOY_TARGET");
    }

    #[test]
    fn var_with_scale_oliver_prefers_scale_in_staging() {
        let _guard = env_lock().lock().expect("env lock");
        env::set_var("DEPLOY_TARGET", "staging");
        env::set_var("TEST_ALIAS_KEY", "base-value");
        env::set_var("SCALE_OLIVER_TEST_ALIAS_KEY", "scale-value");
        let value = var_with_scale_oliver("TEST_ALIAS_KEY").expect("value");
        assert_eq!(value, "scale-value");
        env::remove_var("TEST_ALIAS_KEY");
        env::remove_var("SCALE_OLIVER_TEST_ALIAS_KEY");
        env::remove_var("DEPLOY_TARGET");
    }
}
