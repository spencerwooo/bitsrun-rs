use std::fs;
use std::io;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

/// Campus network user credentials
#[derive(Serialize, Deserialize)]
pub struct BitUser {
    username: Option<String>,
    password: Option<String>,
}

impl BitUser {
    pub fn new(username: Option<String>, password: Option<String>) -> Self {
        Self { username, password }
    }
}

pub fn get_bit_user(
    username: Option<String>,
    password: Option<String>,
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
            print!("Please enter your username: ");
            let mut username = String::new();
            io::stdin().read_line(&mut username)?;
        }
        if user_from_file.password.is_none() {
            let password = rpassword::prompt_password("Please enter your password: ")?;
        }
    }
    Ok(bit_user)
}
