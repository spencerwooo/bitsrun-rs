use std::fs;

use anyhow::Context;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

/// Campus network user credentials
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BitUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl BitUser {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        Self { username, password }
    }
}

/// Get campus network user credentials from command line arguments or config file
pub fn get_bit_user(
    username: Option<String>,
    password: Option<String>,
    config_path: Option<String>,
) -> Result<BitUser> {
    let mut bit_user = BitUser::new(username, password);

    // username and password priority: command line > config file > prompt
    if bit_user.username.is_none() | bit_user.password.is_none() {
        let mut user_from_file = BitUser::default();
        match parse_config_file(config_path) {
            Ok(value) => user_from_file = value,
            Err(e) => eprintln!("{}: {}", "warning".yellow(), e),
        }

        match user_from_file.username {
            Some(username) => bit_user.username.get_or_insert(username),
            None => bit_user.username.get_or_insert_with(|| {
                rprompt::prompt_reply("Please enter your campus id: ")
                    .expect("failed to read username")
            }),
        };

        match user_from_file.password {
            Some(password) => bit_user.password.get_or_insert(password),
            None => bit_user.password.get_or_insert_with(|| {
                rpassword::prompt_password("Please enter your password: ")
                    .expect("failed to read password")
            }),
        };
    }

    Ok(bit_user)
}

/// Parse credentials from config file
fn parse_config_file(config_path: Option<String>) -> Result<BitUser> {
    let config_path = config_path.unwrap_or(String::from("bit-user.json"));

    let user_str_from_file = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read config file `{}`", &config_path.underline()))?;
    let user_from_file = serde_json::from_str::<BitUser>(&user_str_from_file)
        .with_context(|| format!("failed to parse config file `{}`", &config_path.underline()))?;
    Ok(user_from_file)
}
