use crate::config;

use std::fs;

use anyhow::Context;
use anyhow::Result;
use owo_colors::OwoColorize;
use owo_colors::Stream::Stdout;
use serde::Deserialize;
use serde::Serialize;

/// Campus network user credentials that are finalized
#[derive(Debug, Default)]
pub struct BitUser {
    pub username: String,
    pub password: String,
    pub dm: bool,
}

/// Partial campus network user credentials
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BitUserPartial {
    pub username: Option<String>,
    pub password: Option<String>,
    pub dm: Option<bool>,
}

impl BitUserPartial {
    pub fn new(username: &Option<String>, password: &Option<String>, dm: Option<bool>) -> Self {
        Self {
            username: username.clone(),
            password: password.clone(),
            dm,
        }
    }
}

/// Parse bit user credentials from config file
fn parse_bit_user_config(config_path: &Option<String>) -> Result<BitUserPartial> {
    let config = config::validate_config_file(config_path)?;

    let user_str_from_file = fs::read_to_string(&config).with_context(|| {
        format!(
            "failed to read config file `{}`",
            &config.if_supports_color(Stdout, |t| t.underline())
        )
    })?;
    let user_from_file =
        serde_json::from_str::<BitUserPartial>(&user_str_from_file).with_context(|| {
            format!(
                "failed to parse config file `{}`",
                &config.if_supports_color(Stdout, |t| t.underline())
            )
        })?;
    Ok(user_from_file)
}

/// Get campus network user credentials from command line arguments or config file
///
/// Note that when logging out, `password` is not required.
/// In this case, `require_password` should be set to `false`.
pub fn finalize_bit_user(
    username: &Option<String>,
    password: &Option<String>,
    dm: bool,
    config_path: &Option<String>,
    require_password: bool,
) -> Result<BitUser> {
    let mut bit_user = BitUserPartial::new(username, password, Some(dm));

    // username and password priority: command line > config file > prompt
    if bit_user.username.is_none() | (require_password & bit_user.password.is_none()) {
        let mut user_from_file = BitUserPartial::default();
        match parse_bit_user_config(config_path) {
            Ok(value) => user_from_file = value,
            Err(e) => println!(
                "{} {}",
                "warning:".if_supports_color(Stdout, |t| t.yellow()),
                e
            ),
        }

        if user_from_file.dm.is_none() & !dm {
            println!(
                "{} logout endpoint not specified in config file! \
                logging out may encounter unexpected results",
                "warning:".if_supports_color(Stdout, |t| t.yellow()),
            );
            println!(
                "{} if this device is a '{}', explicity specify `{}` to use alternative logout endpoint",
                "warning:".if_supports_color(Stdout, |t| t.yellow()),
                "registered dumb terminal".if_supports_color(Stdout, |t| t.on_yellow()),
                "--dm".if_supports_color(Stdout, |t| t.underline())
            );
        }

        match user_from_file.username {
            Some(username) => bit_user.username.get_or_insert(username),
            None => bit_user.username.get_or_insert_with(|| {
                rprompt::prompt_reply(
                    "-> please enter your campus id: ".if_supports_color(Stdout, |t| t.dimmed()),
                )
                .with_context(|| "failed to read username")
                .unwrap()
            }),
        };

        match user_from_file.password {
            Some(password) => bit_user.password.get_or_insert(password),
            None => bit_user.password.get_or_insert_with(|| {
                if require_password {
                    rpassword::prompt_password(
                        "-> please enter your password: ".if_supports_color(Stdout, |t| t.dimmed()),
                    )
                    .with_context(|| "failed to read password")
                    .unwrap()
                } else {
                    // password is not required when logging out
                    String::from("")
                }
            }),
        };
    }

    Ok(BitUser {
        username: bit_user.username.unwrap_or_default(),
        password: bit_user.password.unwrap_or_default(),
        dm: bit_user.dm.unwrap_or_default(),
    })
}
