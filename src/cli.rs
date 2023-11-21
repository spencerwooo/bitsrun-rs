use std::net::IpAddr;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, about, version, arg_required_else_help = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Your campus username
    #[arg(short, long, global = true)]
    pub username: Option<String>,

    /// Your campus password
    #[arg(short, long, global = true)]
    pub password: Option<String>,

    /// Manually specify IP address (IPv4)
    #[arg(short, long, global = true)]
    pub ip: Option<IpAddr>,

    /// Optionally provide path to the config file
    #[arg(short, long, global = true)]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Login to the campus network
    Login,

    /// Logout from the campus network
    Logout,

    /// Check login status
    Status,
}
