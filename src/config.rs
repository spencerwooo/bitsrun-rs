use std::env;
use std::fs;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use owo_colors::OwoColorize;
use owo_colors::Stream::Stdout;

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

/// Config file validation
pub fn validate_config_file(config_path: &Option<String>) -> Result<String, Error> {
    let mut validated_config_path = String::new();
    match &config_path {
        Some(path) => validated_config_path = path.to_owned(),
        None => {
            for path in enumerate_config_paths() {
                if fs::metadata(&path).is_ok() {
                    validated_config_path = path;
                    break;
                }
            }
        }
    }
    let meta = fs::metadata(&validated_config_path)?;
    if !meta.is_file() {
        return Err(anyhow!(
            "`{}` is not a file",
            &validated_config_path.if_supports_color(Stdout, |t| t.underline())
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
    check_permissions(&validated_config_path, &meta)?;
    if validated_config_path.is_empty() {
        return Err(anyhow!(
            "file `{}` not found, available paths can be found with `{}`",
            "bit-user.json".if_supports_color(Stdout, |t| t.underline()),
            "bitsrun config-paths".if_supports_color(Stdout, |t| t.cyan())
        ));
    }
    Ok(validated_config_path)
}
