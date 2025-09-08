use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Secrets {
    api_password: String,
}

pub(crate) fn load_password(path: &str) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let secrets: Secrets = serde_yaml::from_str(&content).ok()?;
    Some(secrets.api_password)
}