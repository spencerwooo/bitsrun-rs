mod cli;
mod client;
mod config;
mod daemon;
mod tables;
mod user;
mod xencode;

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use cli::ClientArgs;
use cli::StatusArgs;
use owo_colors::OwoColorize;
use owo_colors::Stream::Stderr;
use owo_colors::Stream::Stdout;

use cli::Arguments;
use cli::Commands;
use client::get_login_state;
use client::SrunClient;
use daemon::SrunDaemon;
use tables::print_config_paths;
use tables::print_login_state;

#[tokio::main]
async fn main() {
    if let Err(err) = cli().await {
        eprintln!(
            "{} {}: {}",
            "bitsrun".if_supports_color(Stderr, |t| t.bright_red()),
            "(error)".if_supports_color(Stderr, |t| t.dimmed()),
            err
        );
        std::process::exit(1);
    }
}

async fn cli() -> Result<()> {
    let args = Arguments::parse();

    // reusable http client
    let http_client = reqwest::Client::new();

    // commands
    match &args.command {
        // check login status
        Some(Commands::Status(status_args)) => {
            srun_status(http_client, status_args, args.verbose).await?
        }

        // login or logout
        Some(Commands::Login(client_args)) | Some(Commands::Logout(client_args)) => {
            let bit_user = user::finalize_bit_user(
                &client_args.username,
                &client_args.password,
                client_args.dm,
                &client_args.config,
                matches!(args.command, Some(Commands::Login(_))),
            )
            .with_context(|| "unable to parse user credentials")?;

            let srun_client = SrunClient::new(
                bit_user.username,
                bit_user.password,
                Some(http_client),
                client_args.ip,
                Some(bit_user.dm),
            )
            .await?;

            match &args.command {
                Some(Commands::Login(_)) => {
                    srun_login(&srun_client, client_args, args.verbose).await?
                }
                Some(Commands::Logout(_)) => {
                    srun_logout(&srun_client, client_args, args.verbose).await?
                }
                _ => {}
            };
        }

        Some(Commands::KeepAlive(daemon_args)) => {
            let config_path = daemon_args.config.to_owned();
            let daemon = SrunDaemon::new(config_path)?;
            daemon.start(http_client).await?;
        }

        Some(Commands::ConfigPaths) => print_config_paths(),

        None => {}
    }

    Ok(())
}

async fn srun_status(
    http_client: reqwest::Client,
    status_args: &StatusArgs,
    verbose: bool,
) -> Result<()> {
    // only verbose on args.verbose = true and not outputting json
    let login_state = get_login_state(&http_client, verbose).await?;

    // output json
    if status_args.json & !verbose {
        let raw_json = serde_json::to_string(&login_state)?;
        println!("{}", raw_json);
        return Ok(());
    }

    // output human readable
    match login_state.error.as_str() {
        "ok" => {
            println!(
                "{} {} {} is online",
                "bitsrun:".if_supports_color(Stdout, |t| t.bright_green()),
                &login_state
                    .online_ip
                    .to_string()
                    .if_supports_color(Stdout, |t| t.underline()),
                format!("({})", login_state.user_name.clone().unwrap_or_default())
                    .if_supports_color(Stdout, |t| t.dimmed())
            );

            // print status table
            print_login_state(login_state);
        }
        _ => {
            println!(
                "{} {} is offline",
                "bitsrun:".if_supports_color(Stdout, |t| t.blue()),
                login_state
                    .online_ip
                    .to_string()
                    .if_supports_color(Stdout, |t| t.underline())
            );
        }
    };
    Ok(())
}

async fn srun_login(
    srun_client: &SrunClient,
    client_args: &ClientArgs,
    verbose: bool,
) -> Result<()> {
    let resp = srun_client.login(client_args.force, verbose).await?;
    match resp.error.as_str() {
        "ok" => println!(
            "{} {} {} logged in",
            "bitsrun:".if_supports_color(Stdout, |t| t.bright_green()),
            resp.online_ip
                .to_string()
                .if_supports_color(Stdout, |t| t.underline()),
            format!("({})", resp.username.clone().unwrap_or_default())
                .if_supports_color(Stdout, |t| t.dimmed())
        ),
        _ => println!(
            "{} failed to login, {} {}",
            "bitsrun:".if_supports_color(Stdout, |t| t.red()),
            resp.error,
            format!("({})", resp.error_msg).if_supports_color(Stdout, |t| t.dimmed())
        ),
    };
    Ok(())
}

async fn srun_logout(
    srun_client: &SrunClient,
    client_args: &ClientArgs,
    verbose: bool,
) -> Result<()> {
    let resp = srun_client.logout(client_args.force, verbose).await?;
    match resp.error.as_str() {
        "ok" | "logout_ok" => println!(
            "{} {} logged out",
            "bitsrun:".if_supports_color(Stdout, |t| t.green()),
            resp.online_ip
                .to_string()
                .if_supports_color(Stdout, |t| t.underline())
        ),
        _ => println!(
            "{} failed to logout, {} {}",
            "bitsrun:".if_supports_color(Stdout, |t| t.red()),
            resp.error,
            format!("({})", resp.error_msg).if_supports_color(Stdout, |t| t.dimmed())
        ),
    };
    Ok(())
}
