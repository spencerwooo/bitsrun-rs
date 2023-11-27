use crate::client::SrunLoginState;
use crate::user::enumerate_config_paths;
use chrono::Duration;
use chrono_humanize::Accuracy::Rough;
use chrono_humanize::HumanTime;
use chrono_humanize::Tense::Present;
use humansize::format_size;
use humansize::BINARY;
use owo_colors::OwoColorize;
use owo_colors::Stream::Stdout;
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
    println!(
        "{} list of possible config paths",
        "bitsrun:".if_supports_color(Stdout, |t| t.blue())
    );

    let mut builder = Builder::default();
    builder.set_header(["Priority", "Possible Config Path"]);

    for (i, path) in enumerate_config_paths().iter().enumerate() {
        builder.push_record([(i + 1).to_string(), path.into()]);
    }

    let mut table = builder.build();
    println!("{}", table.with(Style::sharp()));
}

/// Print login state table
///
/// # Example output
///
/// ┌────────────────┬───────────────┬───────────────┬─────────┐
/// │ Traffic Used   │ Online Time   │ User Balance  │ Wallet  │
/// ├────────────────┼───────────────┼───────────────┼─────────┤
/// │ 188.10 GiB     │ 2 months      │ 10.00         │ 0.00    │
/// └────────────────┴───────────────┴───────────────┴─────────┘
pub fn print_login_state(state: SrunLoginState) {
    let mut builder = Builder::default();
    builder.set_header(["Traffic Used", "Online Time", "User Balance", "Wallet"]);

    // parse outputs from login state response
    let traffic_used = state.sum_bytes.unwrap_or(0);

    let online_time = state.sum_seconds.unwrap_or(0);
    let human_time = HumanTime::from(Duration::seconds(online_time));

    let user_balance = state.user_balance.unwrap_or(0) as f32;
    let wallet = state.wallet_balance.unwrap_or(0) as f32;

    builder.push_record([
        format_size(traffic_used, BINARY)
            .if_supports_color(Stdout, |t| t.green())
            .to_string(),
        human_time
            .to_text_en(Rough, Present)
            .if_supports_color(Stdout, |t| t.yellow())
            .to_string(),
        format!("{:.2}", user_balance)
            .if_supports_color(Stdout, |t| t.cyan())
            .to_string(),
        format!("{:.2}", wallet)
            .if_supports_color(Stdout, |t| t.magenta())
            .to_string(),
    ]);

    let mut table = builder.build();
    println!("{}", table.with(Style::sharp()).with(Width::increase(60)));
}
