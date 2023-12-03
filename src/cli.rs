use std::net::IpAddr;

use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, about, version, arg_required_else_help = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Login to the campus network
    Login(ClientArgs),

    /// Logout from the campus network
    Logout(ClientArgs),

    /// Check device login status
    Status(StatusArgs),

    /// List all possible config file paths
    ConfigPaths,
}

#[derive(Args)]
pub struct StatusArgs {
    /// Output JSON literal
    #[arg(short, long)]
    pub json: bool,
}

#[derive(Args)]
pub struct ClientArgs {
    /// Your campus username
    #[arg(short, long)]
    pub username: Option<String>,

    /// Your campus password
    #[arg(short, long)]
    pub password: Option<String>,

    /// Manually specify IP address (IPv4)
    #[arg(long)]
    pub ip: Option<IpAddr>,

    /// Use alternative `dm` logout endpoint for registered dumb terminals
    #[arg(long)]
    pub dm: bool,

    /// Optionally provide path to the config file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Force login/logout, don't check login status
    #[arg(short, long)]
    pub force: bool,
}
