use reqwest;

mod client;

#[tokio::main]
async fn main() {
    let http = reqwest::Client::new();

    let acid = client::get_acid(&http).await;
    println!("acid: {}", acid);

    let login_state = client::get_login_state(&http).await;
    println!("login_state: {:?}", login_state);
}
