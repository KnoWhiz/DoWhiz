use std::env;

fn read_trimmed_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn var_with_scale_oliver(key: &str) -> Option<String> {
    let prefixed = format!("SCALE_OLIVER_{key}");
    read_trimmed_env(&prefixed).or_else(|| read_trimmed_env(key))
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
