use std::env;
use std::fs;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde::Serialize;

/// Campus network user credentials that are finalized
#[derive(Debug, Default)]
pub struct BitUser {
    pub username: String,
    pub password: String,
}

/// Partial campus network user credentials
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BitUserPartial {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl BitUserPartial {
    pub fn new(username: &Option<String>, password: &Option<String>) -> Self {
        Self {
            username: username.clone(),
            password: password.clone(),
        }
    }
}

/// Enumerate possible paths to user config file (platform specific)
///
/// On Windows:
/// * `~\AppData\Roaming\bitsrun\bit-user.json`
///
/// On Linux:
/// * `$XDG_CONFIG_HOME/bitsrun/bit-user.json`
/// * `~/.config/bitsrun/bit-user.json`
/// * `~/.config/bit-user.json`
///
/// On macOS:
/// * `$HOME/Library/Preferences/bitsrun/bit-user.json`
/// * `$HOME/.config/bit-user.json`
/// * `$HOME/.config/bitsrun/bit-user.json`
///
/// Additionally, `bitsrun` will search for config file in the current working directory.
pub fn enumerate_config_paths() -> Vec<String> {
    let mut paths = Vec::new();

    // Windows
    if env::consts::OS == "windows" {
        if let Some(appdata) = env::var_os("APPDATA") {
            paths.push(format!(
                "{}\\bitsrun\\bit-user.json",
                appdata.to_str().unwrap()
            ));
        }
    }

    // Linux (and macOS)
    if let Some(home) = env::var_os("XDG_CONFIG_HOME").or_else(|| env::var_os("HOME")) {
        paths.push(format!("{}/.config/bit-user.json", home.to_str().unwrap()));
        paths.push(format!(
            "{}/.config/bitsrun/bit-user.json",
            home.to_str().unwrap()
        ));
    }

    // macOS
    if env::consts::OS == "macos" {
        if let Some(home) = env::var_os("HOME") {
            paths.push(format!(
                "{}/Library/Preferences/bitsrun/bit-user.json",
                home.to_str().unwrap()
            ));
        }
    }

    // current working directory
    paths.push("bit-user.json".into());
    paths
}

/// Parse credentials from config file
fn parse_config_file(config_path: &Option<String>) -> Result<BitUserPartial> {
    let mut config = String::new();
    if config_path.is_some() {
        config = config_path.clone().unwrap();
    } else {
        for path in enumerate_config_paths() {
            if fs::metadata(&path).is_ok() {
                config = path;
                break;
            }
        }
    }

    if config.is_empty() {
        Err(anyhow!(
            "config file `{}` not found, available paths can be found with `{}`",
            "bit-user.json".underline(),
            "bitsrun config-paths".cyan().bold().underline()
        ))
    } else {
        let user_str_from_file = fs::read_to_string(&config)
            .with_context(|| format!("failed to read config file `{}`", &config.underline()))?;
        let user_from_file = serde_json::from_str::<BitUserPartial>(&user_str_from_file)
            .with_context(|| format!("failed to parse config file `{}`", &config.underline()))?;
        Ok(user_from_file)
    }
}

/// Get campus network user credentials from command line arguments or config file
///
/// Note that when logging out, `password` is not required.
/// In this case, `require_password` should be set to `false`.
pub fn get_bit_user(
    username: &Option<String>,
    password: &Option<String>,
    config_path: &Option<String>,
    require_password: bool,
) -> Result<BitUser> {
    let mut bit_user = BitUserPartial::new(username, password);

    // username and password priority: command line > config file > prompt
    if bit_user.username.is_none() | (require_password & bit_user.password.is_none()) {
        let mut user_from_file = BitUserPartial::default();
        match parse_config_file(config_path) {
            Ok(value) => user_from_file = value,
            Err(e) => println!("{} {}", "warning:".yellow(), e),
        }

        match user_from_file.username {
            Some(username) => bit_user.username.get_or_insert(username),
            None => bit_user.username.get_or_insert_with(|| {
                rprompt::prompt_reply("-> please enter your campus id: ".dimmed())
                    .with_context(|| "failed to read username")
                    .unwrap()
            }),
        };

        match user_from_file.password {
            Some(password) => bit_user.password.get_or_insert(password),
            None => bit_user.password.get_or_insert_with(|| {
                if require_password {
                    rpassword::prompt_password("-> please enter your password: ".dimmed())
                        .with_context(|| "failed to read password")
                        .unwrap()
                } else {
                    "".into()
                }
            }),
        };
    }

    Ok(BitUser {
        username: bit_user.username.unwrap_or_default(),
        password: bit_user.password.unwrap_or_default(),
    })
}
