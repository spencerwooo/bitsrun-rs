use reqwest::Client;
use serde;
use serde::Deserialize;
use url;

/// Constants used for the /srun_portal endpoint
const SRUN_PORTAL: &str = "http://10.0.0.55";
// const SRUN_TYPE: i8 = 1;
// const SRUN_N: i16 = 200;

/// The response from the /rad_user_info endpoint
///
/// This response is used to determine if the device is logged in or not, and if it is logged in,
/// what the current login state is (i.e., IP address, user balance, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct SrunLoginState {
    // always present
    pub error: String,
    pub online_ip: String,

    // present when logged in
    #[serde(rename = "ServerFlag")]
    pub server_flag: Option<i64>,
    pub add_time: Option<i64>,
    pub all_bytes: Option<i64>,
    pub bytes_in: Option<i64>,
    pub bytes_out: Option<i64>,
    pub checkout_date: Option<i64>,
    pub domain: Option<String>,
    pub group_id: Option<String>,
    pub keepalive_time: Option<i64>,
    pub products_name: Option<String>,
    pub real_name: Option<String>,
    pub remain_bytes: Option<i64>,
    pub remain_seconds: Option<i64>,
    pub sum_bytes: Option<i64>,
    pub sum_seconds: Option<i64>,
    pub sysver: Option<String>,
    pub user_balance: Option<i64>,
    pub user_charge: Option<i64>,
    pub user_mac: Option<String>,
    pub user_name: Option<String>,
    pub wallet_balance: Option<i64>,

    // present when logged out
    pub client_ip: Option<String>,
    // pub ecode: Option<i64>,
    pub error_msg: Option<String>,
    pub res: Option<String>,
    pub srun_ver: Option<String>,
    pub st: Option<i64>,
}

/// Get the login state of the current device
pub async fn get_login_state(client: &Client) -> SrunLoginState {
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
    serde_json::from_str::<SrunLoginState>(raw_json).unwrap_or_else(|e| {
        panic!("Failed to parse login state: {}", e);
    })
}

/// Get the ac_id of the current device
async fn get_acid(client: &Client) -> String {
    let resp = client.get(SRUN_PORTAL).send().await.unwrap();
    let redirect_url = resp.url().to_string();
    let parsed_url = url::Url::parse(&redirect_url).unwrap();

    let mut query = parsed_url.query_pairs().into_owned();
    return query
        .find(|(key, _)| key == "ac_id")
        .unwrap_or((String::from(""), String::from("1")))
        .1;
}

/// SRUN portal response type, when calling login/logout
#[derive(Debug, Clone, Deserialize)]
pub struct SrunPortalResponse {
    pub client_ip: String,
    // pub ecode: String,
    pub error: String,
    pub error_msg: String,
    pub online_ip: String,
    pub res: String,
    pub srun_ver: String,

    // present when logging out but failed
    pub st: Option<i64>,
}

/// SRUN client, with a defined username/password and http client
pub struct SrunClient {
    pub username: String,
    pub password: Option<String>,
    pub ac_id: String,
    pub http_client: Client,
}

impl SrunClient {
    /// Create a new SRUN client, where the http client will be reused if provided
    pub async fn new(
        username: String,
        password: Option<String>,
        http_client: Option<Client>,
    ) -> SrunClient {
        let http_client = http_client.unwrap_or(Client::new());
        let ac_id = get_acid(&http_client).await;
        SrunClient {
            username,
            password,
            ac_id,
            http_client,
        }
    }

    /// Login to the SRUN portal
    pub async fn login(&self) {
        // check if already logged in
        let login_state = get_login_state(&self.http_client).await;
        if login_state.error == "ok" {
            println!("Already logged in");
            return;
        }
    }

    /// Logout of the SRUN portal
    pub async fn logout(&self) -> SrunPortalResponse {
        // check if already logged out
        let login_state = get_login_state(&self.http_client).await;
        if login_state.error == "not_online_error" {
            panic!("Already logged out");
        }

        // check if username match
        if login_state.user_name.unwrap() != self.username {
            println!("Warning, username mismatch, things may still work");
        }

        // perform logout action
        let params = [
            ("callback", "jsonp"),
            ("action", "logout"),
            ("ip", login_state.online_ip.as_str()),
            ("ac_id", self.ac_id.as_str()),
            ("username", self.username.as_str()),
        ];
        let url = format!("{}/cgi-bin/srun_portal", SRUN_PORTAL);

        let resp = self
            .http_client
            .get(&url)
            .query(&params)
            .send()
            .await
            .unwrap_or_else(|e| {
                panic!("Failed to logout: {}", e);
            });
        let raw_text = resp.text().await.unwrap();
        let raw_json = &raw_text[6..raw_text.len() - 1];
        serde_json::from_str::<SrunPortalResponse>(raw_json).unwrap_or_else(|e| {
            panic!("Failed to parse logout response: {}", e);
        })
    }
}
