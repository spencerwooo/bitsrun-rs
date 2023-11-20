mod client;

use client::get_login_state;
use client::SrunClient;

#[tokio::main]
async fn main() {
    let http_client = reqwest::Client::new();

    let login_state = get_login_state(&http_client).await;
    println!("{:?}", login_state);

    let srun_client = SrunClient::new(
        String::from("3120225654"),
        Some(String::from("password")),
        Some(http_client.clone()),
    )
    .await;

    srun_client.login().await;
    srun_client.logout().await;
    srun_client.login().await;
}
