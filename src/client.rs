use reqwest::Client;
use serde;
use serde::Deserialize;
use url;

// Constants used for the /get-challenge endpoint
const SRUN_PORTAL: &str = "http://10.0.0.55";
// const SRUN_TYPE: i8 = 1;
// const SRUN_N: i16 = 200;

#[derive(Debug, Clone, Deserialize)]
pub struct SrunLoginState {
    #[serde(rename = "ServerFlag")]
    pub server_flag: i64,
    pub add_time: i64,
    pub all_bytes: i64,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub checkout_date: i64,
    pub domain: String,
    pub error: String,
    pub group_id: String,
    pub keepalive_time: i64,
    pub online_ip: String,
    pub products_name: String,
    pub real_name: String,
    pub remain_bytes: i64,
    pub remain_seconds: i64,
    pub sum_bytes: i64,
    pub sum_seconds: i64,
    pub sysver: String,
    pub user_balance: i64,
    pub user_charge: i64,
    pub user_mac: String,
    pub user_name: String,
    pub wallet_balance: i64,
}

/// Get the login state of the current device
pub async fn get_login_state(client: &Client) -> Result<SrunLoginState, serde_json::Error> {
    // call /rad_user_info with callback=jsonp to get the login state
    let params = [("callback", "jsonp")];
    let url = format!("{}/cgi-bin/rad_user_info", SRUN_PORTAL);

    // get the response and extract the json
    let resp = client
        .get(&url)
        .query(&params)
        .send()
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to get login state: {}", e);
        });
    let text = resp.text().await.unwrap();

    // valid json starts at index 6 and ends at the second to last character
    let raw_json = &text[6..text.len() - 1];
    serde_json::from_str::<SrunLoginState>(raw_json)
}

pub async fn get_acid(client: &Client) -> String {
    let resp = client.get(SRUN_PORTAL).send().await.unwrap();
    let redirect_url = resp.url().to_string();
    let parsed_url = url::Url::parse(&redirect_url).unwrap();

    let mut query = parsed_url.query_pairs().into_owned();
    return query.find(|(key, _)| key == "ac_id").unwrap().1;
}

struct SrunClient {
    username: Option<String>,
    password: Option<String>,
}

impl SrunClient {
    pub fn new() -> SrunClient {
        SrunClient {
            username: None,
            password: None,
        }
    }

    pub fn login(&self) {}

    pub fn logout(&self) {}
}
