use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub type BoxError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Deserialize)]
pub struct EmployeeConfigFile {
    #[serde(default)]
    pub default_employee_id: Option<String>,
    #[serde(default)]
    pub employees: Vec<EmployeeConfigEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmployeeConfigEntry {
    pub id: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub runner: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub addresses: Vec<String>,
    #[serde(default)]
    pub agents_path: Option<PathBuf>,
    #[serde(default)]
    pub claude_path: Option<PathBuf>,
    #[serde(default)]
    pub soul_path: Option<PathBuf>,
    #[serde(default)]
    pub skills_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct EmployeeProfile {
    pub id: String,
    pub display_name: Option<String>,
    pub runner: String,
    pub model: Option<String>,
    pub addresses: Vec<String>,
    pub address_set: HashSet<String>,
    pub agents_path: Option<PathBuf>,
    pub claude_path: Option<PathBuf>,
    pub soul_path: Option<PathBuf>,
    pub skills_dir: Option<PathBuf>,
}

impl EmployeeProfile {
    pub fn matches_address(&self, address: &str) -> bool {
        let normalized = normalize_address(address);
        self.address_set.contains(&normalized)
    }
}

#[derive(Debug, Clone)]
pub struct EmployeeDirectory {
    pub employees: Vec<EmployeeProfile>,
    pub employee_by_id: HashMap<String, EmployeeProfile>,
    pub default_employee_id: Option<String>,
    pub service_addresses: HashSet<String>,
}

impl EmployeeDirectory {
    pub fn employee(&self, id: &str) -> Option<&EmployeeProfile> {
        self.employee_by_id.get(id)
    }

    pub fn employee_ids(&self) -> Vec<String> {
        self.employees.iter().map(|emp| emp.id.clone()).collect()
    }
}

pub fn load_employee_directory(config_path: &Path) -> Result<EmployeeDirectory, BoxError> {
    let content = fs::read_to_string(config_path)?;
    let parsed: EmployeeConfigFile = toml::from_str(&content)?;
    let base_dir = config_path.parent().unwrap_or_else(|| Path::new("."));

    if parsed.employees.is_empty() {
        return Err("employee.toml must include at least one employee".into());
    }

    let mut employees = Vec::new();
    let mut employee_by_id = HashMap::new();
    let mut service_addresses = HashSet::new();

    for entry in parsed.employees.iter() {
        if entry.addresses.is_empty() {
            return Err(format!("employee '{}' must list at least one address", entry.id).into());
        }
        let runner = normalize_runner(entry.runner.as_deref());
        let addresses: Vec<String> = entry
            .addresses
            .iter()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect();
        if addresses.is_empty() {
            return Err(format!("employee '{}' has empty addresses", entry.id).into());
        }
        let address_set: HashSet<String> = addresses
            .iter()
            .map(|value| normalize_address(value))
            .collect();
        service_addresses.extend(address_set.iter().cloned());

        let profile = EmployeeProfile {
            id: entry.id.trim().to_string(),
            display_name: entry
                .display_name
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string()),
            runner,
            model: entry
                .model
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string()),
            addresses,
            address_set,
            agents_path: resolve_optional_path(base_dir, entry.agents_path.as_ref()),
            claude_path: resolve_optional_path(base_dir, entry.claude_path.as_ref()),
            soul_path: resolve_optional_path(base_dir, entry.soul_path.as_ref()),
            skills_dir: resolve_optional_path(base_dir, entry.skills_dir.as_ref()),
        };

        employee_by_id.insert(profile.id.clone(), profile.clone());
        employees.push(profile);
    }

    Ok(EmployeeDirectory {
        employees,
        employee_by_id,
        default_employee_id: parsed.default_employee_id,
        service_addresses,
    })
}

fn normalize_runner(raw: Option<&str>) -> String {
    raw.unwrap_or("codex").trim().to_ascii_lowercase()
}

fn normalize_address(address: &str) -> String {
    address.trim().to_ascii_lowercase()
}

fn resolve_optional_path(base_dir: &Path, value: Option<&PathBuf>) -> Option<PathBuf> {
    value.map(|path| resolve_path(base_dir, path))
}

fn resolve_path(base_dir: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    }
}
