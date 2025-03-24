use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AccountConfig {
    pub ssh_key: String,
    pub git_username: String,
    pub git_email: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub accounts: HashMap<String, AccountConfig>,
}
