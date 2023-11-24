use std::fs;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

/// Campus network user credentials
#[derive(Serialize, Deserialize)]
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
    username: &mut Option<String>,
    password: &mut Option<String>,
    config_path: Option<String>,
) -> Result<BitUser> {
    let config_path = config_path.unwrap_or(String::from("bit-user.json"));

    if username.is_none() | password.is_none() {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_path)?;

        let user_from_file: BitUser = serde_json::from_reader(file)
            .with_context(|| "failed to read config file")
            .unwrap();

        if user_from_file.username.is_none() {
            let input_name = rprompt::prompt_reply("Please enter your campus id: ")
                .with_context(|| "failed to read username")?;
            username.get_or_insert(input_name);
        }
        if user_from_file.password.is_none() {
            let input_password = rpassword::prompt_password("Please enter your password: ")
                .with_context(|| "failed to read password")?;
            password.get_or_insert(input_password);
        }
    }

    Ok(BitUser::new(username.clone(), password.clone()))
}
