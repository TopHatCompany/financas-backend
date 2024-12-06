use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    pub port: u16,
    #[serde(default = "default_client_host")]
    pub client_host: String,
    pub database_url: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_client_host() -> String {
    "http://localhost:5123".to_string()
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("Provide missing environment variables for Config")
    }
}

#[derive(Serialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    pub message: String,
}
