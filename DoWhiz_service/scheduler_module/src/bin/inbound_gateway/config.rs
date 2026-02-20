use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Default)]
pub(super) struct GatewayConfigFile {
    #[serde(default)]
    pub(super) server: GatewayServerConfig,
    #[serde(default)]
    pub(super) defaults: GatewayDefaultsConfig,
    #[serde(default)]
    pub(super) routes: Vec<GatewayRouteConfig>,
}

#[derive(Debug, Deserialize, Default)]
pub(super) struct GatewayServerConfig {
    pub(super) host: Option<String>,
    pub(super) port: Option<u16>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub(super) struct GatewayDefaultsConfig {
    pub(super) tenant_id: Option<String>,
    pub(super) employee_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub(super) struct GatewayRouteConfig {
    pub(super) channel: String,
    pub(super) key: String,
    pub(super) employee_id: String,
    pub(super) tenant_id: Option<String>,
}

pub(super) fn resolve_gateway_config_path() -> Result<PathBuf, String> {
    if let Ok(path) = env::var("GATEWAY_CONFIG_PATH") {
        if !path.trim().is_empty() {
            return Ok(PathBuf::from(path));
        }
    }

    let cwd = env::current_dir().map_err(|err| err.to_string())?;
    let direct = cwd.join("gateway.toml");
    if direct.exists() {
        return Ok(direct);
    }

    let nested = cwd.join("DoWhiz_service").join("gateway.toml");
    if nested.exists() {
        return Ok(nested);
    }

    Err("GATEWAY_CONFIG_PATH not set and gateway.toml not found".to_string())
}

pub(super) fn resolve_employee_config_path() -> PathBuf {
    if let Ok(path) = env::var("EMPLOYEE_CONFIG_PATH") {
        if !path.trim().is_empty() {
            return PathBuf::from(path);
        }
    }

    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let direct = cwd.join("employee.toml");
    if direct.exists() {
        return direct;
    }
    let nested = cwd.join("DoWhiz_service").join("employee.toml");
    if nested.exists() {
        return nested;
    }

    PathBuf::from("DoWhiz_service/employee.toml")
}

pub(super) fn load_gateway_config(path: &Path) -> Result<GatewayConfigFile, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|err| format!("failed to read gateway config: {}", err))?;
    toml::from_str::<GatewayConfigFile>(&content)
        .map_err(|err| format!("failed to parse gateway config: {}", err))
}
