mod cli;
mod client;
mod tables;
mod user;
mod xencode;

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;
use owo_colors::Stream::Stderr;
use owo_colors::Stream::Stdout;

use cli::Arguments;
use cli::Commands;
use client::get_login_state;
use client::SrunClient;
use tables::print_config_paths;
use tables::print_login_state;

#[tokio::main]
async fn main() {
    if let Err(err) = cli().await {
        eprintln!(
            "{} {} {}",
            "bitsrun:".if_supports_color(Stderr, |t| t.bright_red()),
            "[error]".if_supports_color(Stderr, |t| t.dimmed()),
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
            let login_state = get_login_state(&http_client).await?;

            // output json
            if status_args.json {
                let raw_json = serde_json::to_string(&login_state)?;
                println!("{}", raw_json);
                return Ok(());
            }

            // output human readable
            if login_state.error == "ok" {
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
            } else {
                println!(
                    "{} {} is offline",
                    "bitsrun:".if_supports_color(Stdout, |t| t.blue()),
                    login_state
                        .online_ip
                        .to_string()
                        .if_supports_color(Stdout, |t| t.underline())
                );
            }
        }

        // login or logout
        Some(Commands::Login(client_args)) | Some(Commands::Logout(client_args)) => {
            let bit_user = user::get_bit_user(
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
                Some(client_args.dm),
            )
            .await?;

            if matches!(args.command, Some(Commands::Login(_))) {
                let resp = srun_client.login().await?;
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
                }

                if args.verbose {
                    let pretty_json = serde_json::to_string_pretty(&resp)?;
                    println!(
                        "{} response from API\n{}",
                        "bitsrun:".if_supports_color(Stdout, |t| t.blue()),
                        pretty_json
                    );
                }
            } else if matches!(args.command, Some(Commands::Logout(_))) {
                let resp = srun_client.logout().await?;
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
                }

                if args.verbose {
                    let pretty_json = serde_json::to_string_pretty(&resp)?;
                    println!(
                        "{} response from API\n{}",
                        "bitsrun:".if_supports_color(Stdout, |t| t.blue()),
                        pretty_json
                    );
                }
            }
        }

        Some(Commands::ConfigPaths) => print_config_paths(),

        None => {}
    }

    Ok(())
}
