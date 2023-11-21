mod client;
mod xencode;

use client::get_login_state;
use client::SrunClient;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let login_state = get_login_state(&http_client).await;
    println!("{:?}", login_state);

    let srun_client = SrunClient::new(
        String::from("id"),
        Some(String::from("pass")),
        Some(http_client),
        None,
    )
    .await;

    srun_client.logout().await;
    srun_client.login().await;
}
