use std::env;
use std::fs;

use anyhow::anyhow;
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
    match &config_path {
        Some(path) => config = path.to_owned(),
        None => {
            for path in enumerate_config_paths() {
                if fs::metadata(&path).is_ok() {
                    config = path;
                    break;
                }
            }
        }
    }

    // check if file is valid (i.e., is a file and permissions are not too open)
    let meta = fs::metadata(&config)?;
    if !meta.is_file() {
        return Err(anyhow!(
            "`{}` is not a file",
            &config.if_supports_color(Stdout, |t| t.underline())
        ));
    }

    // file should only be read/writeable by the owner alone, i.e., 0o600
    // note: this check is only performed on unix systems
    #[cfg(unix)]
    fn check_permissions(config: &String, meta: &std::fs::Metadata) -> Result<(), anyhow::Error> {
        use std::os::unix::fs::MetadataExt;
        if meta.mode() & 0o777 != 0o600 {
            return Err(anyhow!(
                "`{}` has too open permissions {}, aborting!\n\
                {}: set permissions to {} with `chmod 600 {}`",
                &config.if_supports_color(Stdout, |t| t.underline()),
                (meta.mode() & 0o777)
                    .to_string()
                    .if_supports_color(Stdout, |t| t.on_red()),
                "tip".if_supports_color(Stdout, |t| t.green()),
                "600".if_supports_color(Stdout, |t| t.on_cyan()),
                &config
            ));
        }
        Ok(())
    }

    #[cfg(windows)]
    #[allow(unused)]
    fn check_permissions(_config: &str, _meta: &std::fs::Metadata) -> Result<(), anyhow::Error> {
        // Windows doesn't support Unix-style permissions, so we'll just return Ok here.
        Ok(())
    }

    check_permissions(&config, &meta)?;

    // check if file is empty
    if config.is_empty() {
        return Err(anyhow!(
            "file `{}` not found, available paths can be found with `{}`",
            "bit-user.json".if_supports_color(Stdout, |t| t.underline()),
            "bitsrun config-paths".if_supports_color(Stdout, |t| t.cyan())
        ));
    }

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
pub fn get_bit_user(
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
        match parse_config_file(config_path) {
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
