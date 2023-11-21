use std::net::IpAddr;

use crate::xencode::fkbase64;
use crate::xencode::xencode;
use hmac::Hmac;
use hmac::Mac;
use md5::Digest;
use md5::Md5;
use reqwest::Client;
use serde;
use serde::Deserialize;
use serde::Serialize;
use sha1::Sha1;
use url;

/// Constants used for the /srun_portal endpoint
const SRUN_PORTAL: &str = "http://10.0.0.55";
const SRUN_TYPE: &str = "1";
const SRUN_N: &str = "200";

/// The response from the /rad_user_info endpoint
///
/// This response is used to determine if the device is logged in or not, and if it is logged in,
/// what the current login state is (i.e., IP address, user balance, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct SrunLoginState {
    // always present
    pub error: String,
    pub online_ip: IpAddr,

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
    pub client_ip: Option<IpAddr>,
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
    // present when logging out
    pub client_ip: IpAddr,
    // pub ecode: String,
    pub error: String,
    pub error_msg: String,
    pub online_ip: IpAddr,
    pub res: String,
    pub srun_ver: String,

    // present when logging out but failed
    pub st: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SrunChallenge {
    pub challenge: String,
    // pub client_ip: String,
    // pub ecode: i64,
    // pub error: String,
    // pub error_msg: String,
    // pub expire: String,
    // pub online_ip: String,
    // pub res: String,
    // pub srun_ver: String,
    // pub st: i64,
}

/// SRUN client
pub struct SrunClient {
    // reusable http client
    pub http_client: Client,

    // srun login info, username is student id
    pub username: String,
    pub password: Option<String>,

    // srun portal info
    pub ip: IpAddr,
    pub ac_id: String,
    pub login_state: SrunLoginState,
}

impl SrunClient {
    /// Create a new SRUN client, where the http client will be reused if provided
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the SRUN account (student id)
    /// * `password` - The password of the SRUN account
    /// * `ip` - The IP address (`online_ip` from the login portal if not specified)
    /// * `http_client` - The http client to be used (a new one will be created if not specified)
    pub async fn new(
        username: String,
        password: Option<String>,
        http_client: Option<Client>,
        ip: Option<IpAddr>,
    ) -> SrunClient {
        let http_client = http_client.unwrap_or(Client::new());
        let ac_id = get_acid(&http_client).await;
        let login_state = get_login_state(&http_client).await;
        let ip = ip.unwrap_or(login_state.online_ip.clone());
        SrunClient {
            http_client,
            username,
            password,
            ip,
            ac_id,
            login_state,
        }
    }

    /// Login to the SRUN portal
    pub async fn login(&self) {
        // check if already logged in
        // if self.login_state.error == "ok" {
        //     panic!("Already logged in");
        // }

        // check if password provided
        if self.password.is_none() {
            panic!("Password not provided");
        }

        // construct checksum and crypto encodings
        let token = self.get_challenge().await;

        #[derive(Serialize, Deserialize)]
        struct ChksumData {
            username: String,
            password: String,
            ip: String,
            acid: String,
            enc_ver: String,
        }
        let chksum_data = ChksumData {
            username: self.username.clone(),
            password: self.password.as_ref().unwrap().clone(),
            ip: self.ip.to_string(),
            acid: self.ac_id.clone(),
            enc_ver: String::from("srun_bx1"),
        };

        let json_chksum_data = serde_json::to_string(&chksum_data).unwrap();
        let encoded_data = xencode(&json_chksum_data.as_str(), &token.as_str());
        let info = format!("{}{}", "{SRBX1}", fkbase64(encoded_data));

        // construct param payload
        let mac = Hmac::<Md5>::new_from_slice(token.as_bytes()).unwrap();
        let hmd5 = format!("{:x}", mac.finalize().into_bytes());

        let chksum = {
            let chk = format!(
                "{0}{1}{0}{2}{0}{3}{0}{4}{0}{5}{0}{6}{0}{7}",
                &token, &self.username, &hmd5, &self.ac_id, &self.ip, &SRUN_N, &SRUN_TYPE, &info
            );
            let mut hasher = Sha1::new();
            hasher.update(chk);
            format!("{:x}", hasher.finalize())
        };

        // construct request body
        let password_encoded = format!("{}{}", "{MD5}", hmd5);
        let params = [
            ("callback", "jsonp"),
            ("action", "login"),
            ("username", self.username.as_str()),
            ("password", password_encoded.as_str()),
            ("chksum", &chksum.as_str()),
            ("info", &info.as_str()),
            ("ac_id", self.ac_id.as_str()),
            ("ip", &self.ip.to_string()),
            ("type", SRUN_TYPE),
            ("n", SRUN_N),
        ];
        let url = format!("{}/cgi-bin/srun_portal", SRUN_PORTAL);

        // send login request
        let resp = self
            .http_client
            .get(&url)
            .query(&params)
            .send()
            .await
            .unwrap_or_else(|e| {
                panic!("Failed to login: {}", e);
            });
        let raw_text = resp.text().await.unwrap();
        println!("{}", raw_text);
    }

    /// Logout of the SRUN portal
    pub async fn logout(&self) -> SrunPortalResponse {
        // check if already logged out
        if self.login_state.error == "not_online_error" {
            panic!("Already logged out");
        }

        // check if username match
        let logged_in_username = self.login_state.user_name.as_ref().unwrap();
        if logged_in_username != &self.username {
            println!(
                "Warning, logged in user ({}) does not match yourself ({})",
                logged_in_username, self.username
            );
        }

        // check if ip match
        let logged_in_ip = self.login_state.online_ip;
        if logged_in_ip != self.ip {
            println!(
                "Warning, logged in ip ({}) does not match yourself ({})",
                logged_in_ip, self.ip
            );
        }

        // perform logout action
        let params = [
            ("callback", "jsonp"),
            ("action", "logout"),
            ("ip", &self.ip.to_string()),
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

    async fn get_challenge(&self) -> String {
        let params = [
            ("callback", "jsonp"),
            ("username", self.username.as_str()),
            ("ip", &self.ip.to_string()),
        ];
        let url = format!("{}/cgi-bin/get_challenge", SRUN_PORTAL);

        let resp = self
            .http_client
            .get(&url)
            .query(&params)
            .send()
            .await
            .unwrap_or_else(|e| {
                panic!("Failed to get challenge: {}", e);
            });
        let raw_text = resp.text().await.unwrap();
        let raw_json = &raw_text[6..raw_text.len() - 1];
        let parsed_json = serde_json::from_str::<SrunChallenge>(raw_json).unwrap_or_else(|e| {
            panic!("Failed to parse challenge: {}", e);
        });
        parsed_json.challenge
    }
}
