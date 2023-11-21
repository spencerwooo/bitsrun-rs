mod cli;
mod client;
mod xencode;

use cli::Args;
use cli::Commands;
use client::get_login_state;
use client::SrunClient;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // reusable http client
    let http_client = reqwest::Client::new();

    // commands
    match &args.command {
        Some(Commands::Status) => {
            let login_state = get_login_state(&http_client).await;
            println!("Status: {:?}", login_state);
        }
        Some(Commands::Login) | Some(Commands::Logout) => {
            // parse options
            let username = args.username.unwrap_or_else(|| {
                println!("Please provide username");
                std::process::exit(1);
            });
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
