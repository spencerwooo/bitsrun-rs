use reqwest::Client;
use url;

const SRUN_PORTAL: &str = "http://10.0.0.55";

async fn get_acid(client: &Client) -> String {
    let resp = client.get(SRUN_PORTAL).send().await.unwrap();
    let redirect_url = resp.url().to_string();
    let parsed_url = url::Url::parse(&redirect_url).unwrap();

    let mut query = parsed_url.query_pairs().into_owned();
    return query.find(|(key, _)| key == "ac_id").unwrap().1;
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let acid = get_acid(&client).await;

    println!("acid: {}", acid);
}
