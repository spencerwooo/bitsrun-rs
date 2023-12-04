use crate::client::SrunClient;
use crate::config;

use std::fs;

use anyhow::Context;
use anyhow::Result;
use log::info;
use log::warn;
use owo_colors::OwoColorize;
use owo_colors::Stream::Stdout;

use reqwest::Client;
use serde::Deserialize;

use tokio::signal::ctrl_c;
use tokio::time::Duration;

#[derive(Debug, Deserialize)]
pub struct SrunDaemon {
    username: String,
    password: String,
    dm: bool,
    // polls every 1 hour by default
    poll_interval: Option<u64>,
}

impl SrunDaemon {
    pub fn new(config_path: Option<String>) -> Result<SrunDaemon> {
        let finalized_cfg = config::validate_config_file(&config_path)?;

        // in daemon mode, bitsrun must be able to read all required fields from the config file,
        // including `username`, `password`, and `dm`.
        let daemon_cfg_str = fs::read_to_string(&finalized_cfg).with_context(|| {
            format!(
                "failed to read config file `{}`",
                &finalized_cfg.if_supports_color(Stdout, |t| t.underline())
            )
        })?;
        let daemon_cfg =
            serde_json::from_str::<SrunDaemon>(&daemon_cfg_str).with_context(|| {
                format!(
                    "failed to parse config file `{}`",
                    &finalized_cfg.if_supports_color(Stdout, |t| t.underline())
                )
            })?;

        Ok(daemon_cfg)
    }

    pub async fn start(&self, http_client: Client) -> Result<()> {
        // set logger to INFO level by default
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Info)
            .init();

        // set default polling intervals every 1 hour
        let poll_interval = self.poll_interval.unwrap_or(3600);

        // warn if polling interval is too short
        if poll_interval < 60 * 10 {
            warn!("polling interval is too short, please set it to at least 10 minutes (600s)");
        }

        // start daemon
        let mut srun_ticker = tokio::time::interval(Duration::from_secs(poll_interval));
        let srun = SrunClient::new(
            self.username.clone(),
            self.password.clone(),
            Some(http_client),
            None,
            Some(self.dm),
        )
        .await?;

        info!(
            "starting daemon ({}) with polling interval={}s",
            self.username, poll_interval,
        );

        loop {
            let tick = srun_ticker.tick();
            let login = srun.login(true, false);

            tokio::select! {
                _ = tick => {
                    match login.await {
                        Ok(resp) => {
                            match resp.error.as_str() {
                                "ok" => {
                                    info!("{} ({}): login success, {}", resp.client_ip, self.username, resp.suc_msg.unwrap_or_default());
                                }
                                _ => {
                                    warn!("{} ({}): login failed, {}", resp.client_ip, self.username, resp.error);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("{}: login failed: {}", self.username, e);
                        }
                    }
                }
                _ = ctrl_c() => {
                    info!("{}: gracefully exiting", self.username);
                    break;
                }
            }
        }

        Ok(())
    }
}
