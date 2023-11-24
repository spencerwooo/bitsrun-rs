mod cli;
mod client;
mod user;
mod xencode;

use anyhow::Context;
use cli::Args;
use cli::Commands;
use client::get_login_state;
use client::SrunClient;
use colored::Colorize;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let name = "bitsrun:".blue();

    // reusable http client
    let http_client = reqwest::Client::new();

    // commands
    match &args.command {
        // check login status
        Some(Commands::Status) => {
            let login_state = get_login_state(&http_client).await;
            match login_state {
                Ok(login_state) => {
                    if login_state.error == "ok" {
                        println!(
                            "{} {} ({}) {}",
                            &name.green(),
                            &login_state.online_ip,
                            &login_state.user_name.as_ref().unwrap(),
                            "is online"
                        );
                    } else {
                        println!("{}: {} {}", name, login_state.online_ip, "is offline");
                    }
                    if args.verbose {
                        println!("{:?}", login_state);
                    }
                }
                Err(e) => println!("{} {}", "error:".red(), e),
            }
        }

        // login or logout
        Some(Commands::Login) | Some(Commands::Logout) => {
            let bit_user =
                user::get_bit_user(args.username.clone(), args.password.clone(), args.config)
                    .with_context(|| "unable to parse user credentials")
                    .unwrap();
            println!("{:?}", bit_user);

            let srun_client = SrunClient::new(
                bit_user.username,
                bit_user.password,
                Some(http_client),
                args.ip,
            )
            .await;

            match srun_client {
                Ok(srun_client) => {
                    if let Some(Commands::Login) = &args.command {
                        let resp = srun_client.login().await;
                        match resp {
                            Ok(resp) => println!("{:?}", resp),
                            Err(e) => println!("{} {}", "error:".red(), e),
                        }
                    } else if let Some(Commands::Logout) = &args.command {
                        let resp = srun_client.logout().await;
                        match resp {
                            Ok(resp) => println!("{:?}", resp),
                            Err(e) => println!("{} {}", "error:".red(), e),
                        }
                    }
                }
                Err(e) => println!("{} {}", "error:".red(), e),
            }
        }

        None => {}
    }
}
