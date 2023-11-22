mod cli;
mod client;
mod user;
mod xencode;

use cli::Args;
use cli::Commands;
use client::get_login_state;
use client::SrunClient;
use colored::Colorize;
use serde_json;
use std::fs::File;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let name = "bitsrun".blue();

    // reusable http client
    let http_client = reqwest::Client::new();

    // commands
    match &args.command {
        // check login status
        Some(Commands::Status) => {
            let login_state = get_login_state(&http_client).await;
            if login_state.error == "ok" {
                println!(
                    "{}: {} ({}) {}",
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

        // login or logout
        Some(Commands::Login) | Some(Commands::Logout) => {
            // if username or password is not provided, try to read from config file
            let bit_user = user::get_bit_user(
                args.username.clone(),
                args.password.clone(),
                args.config.clone(),
            );

            // parse options
            let username = args.username;
            let password = args.password;
            let ip = args.ip;
            // let config_path = args.config;

            let srun_client = SrunClient::new(username, password, Some(http_client), ip).await;

            if let Some(Commands::Login) = &args.command {
                let resp = srun_client.login().await;
                println!("{:?}", resp);
            } else if let Some(Commands::Logout) = &args.command {
                let resp = srun_client.logout().await;
                println!("{:?}", resp);
            }
        }

        None => {}
    }
}
