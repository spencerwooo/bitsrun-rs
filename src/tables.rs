use crate::client::SrunLoginState;
use crate::user::enumerate_config_paths;
use owo_colors::OwoColorize;
use tabled::builder::Builder;
use tabled::settings::Style;
use tabled::settings::Width;

/// Print all possible config file paths as a table
///
/// # Example output
///
/// ┌──────────┬───────────────────────────────────────────────────────────┐
/// │ Priority │ Possible config path                                      │
/// ├──────────┼───────────────────────────────────────────────────────────┤
/// │ 1        │ C:\Users\{USERNAME}\AppData\Roaming\bitsrun\bit-user.json │
/// └──────────┴───────────────────────────────────────────────────────────┘
pub fn print_config_paths() {
    println!("{} list of possible config paths", "bitsrun:".blue());

    let mut builder = Builder::default();
    builder.set_header(["Priority", "Possible Config Path"]);

    for (i, path) in enumerate_config_paths().iter().enumerate() {
        builder.push_record([(i + 1).to_string(), path.into()]);
    }

    let mut table = builder.build();
    println!("{}", table.with(Style::sharp()));
}

/// Print login state table
pub fn print_login_state(state: SrunLoginState) {
    let mut builder = Builder::default();
    builder.set_header(["Traffic Used", "Online Time", "User Balance", "Wallet"]);

    // parse outputs from login state response
    let traffic_used = state.sum_bytes.unwrap_or(0);
    let online_time = state.sum_seconds.unwrap_or(0);
    let user_balance = state.user_balance.unwrap_or(0);
    let wallet = state.wallet_balance.unwrap_or(0);
    builder.push_record(vec![
        traffic_used.green().to_string(),
        online_time.yellow().to_string(),
        user_balance.cyan().to_string(),
        wallet.magenta().to_string(),
    ]);

    let mut table = builder.build();
    println!("{}", table.with(Style::sharp()).with(Width::increase(60)));
}
