use reqwest::Client;
use url;

// Constants used for the /get-challenge endpoint
const SRUN_PORTAL: &str = "http://10.0.0.55";
// const SRUN_TYPE: i8 = 1;
// const SRUN_N: i16 = 200;

pub async fn get_acid(client: &Client) -> String {
    let resp = client.get(SRUN_PORTAL).send().await.unwrap();
    let redirect_url = resp.url().to_string();
    let parsed_url = url::Url::parse(&redirect_url).unwrap();

    let mut query = parsed_url.query_pairs().into_owned();
    return query.find(|(key, _)| key == "ac_id").unwrap().1;
}

pub async fn get_login_state(client: &Client) {
    let params = [("callback", "jsonp")];
    let url = format!("{}/cgi-bin/rad_user_info", SRUN_PORTAL);

    let resp = client.get(&url).query(&params).send().await.unwrap();
    let text = resp.text().await.unwrap()[6..].to_string();

    println!("{}", text);
}
