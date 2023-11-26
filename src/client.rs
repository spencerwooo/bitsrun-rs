use std::net::IpAddr;

use crate::xencode::fkbase64;
use crate::xencode::xencode;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use hmac::Hmac;
use hmac::Mac;
use md5::Digest;
use md5::Md5;
use owo_colors::OwoColorize;
use reqwest::Client;

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use sha1::Sha1;

/// Constants used for the /srun_portal endpoint
pub const SRUN_PORTAL: &str = "http://10.0.0.55";
pub const SRUN_TYPE: &str = "1";
pub const SRUN_N: &str = "200";

/// The response from the `/rad_user_info` endpoint
///
/// This response is used to determine if the device is logged in or not, and if it is logged in,
/// what the current login state is (i.e., IP address, user balance, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrunLoginState {
    // always present
    pub error: String,
    pub online_ip: IpAddr,

    // present when logged in
    #[serde(rename = "ServerFlag", skip_serializing_if = "Option::is_none")]
    pub server_flag: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepalive_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_charge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_balance: Option<i64>,

    // present when logged out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<IpAddr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srun_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub st: Option<i64>,
}

/// Get the login state of the current device
pub async fn get_login_state(client: &Client) -> Result<SrunLoginState> {
    // call /rad_user_info with callback=jsonp to get the login state
    let params = [("callback", "jsonp")];
    let url = format!("{}/cgi-bin/rad_user_info", SRUN_PORTAL);

    // get the response and extract the json
    let resp = client
        .get(&url)
        .query(&params)
        .send()
        .await
        .with_context(|| "failed to get login state")?;
    let text = resp.text().await?;

    // valid json starts at index 6 and ends at the second to last character
    if text.len() < 8 {
        bail!("login status response too short: `{}`", text)
    }
    let raw_json = &text[6..text.len() - 1];
    let parsed_json = serde_json::from_str::<SrunLoginState>(raw_json).with_context(|| {
        format!(
            "failed to parse malformed login status response:\n  {}",
            raw_json
        )
    })?;

    Ok(parsed_json)
}

/// Get the ac_id of the current device
async fn get_acid(client: &Client) -> Result<String> {
    let resp = client
        .get(SRUN_PORTAL)
        .send()
        .await
        .with_context(|| format!("failed to get ac_id from `{}`", SRUN_PORTAL.underline()))?;
    let redirect_url = resp.url().to_string();
    let parsed_url = url::Url::parse(&redirect_url)
        .with_context(|| format!("failed to parse url `{}`", redirect_url.underline()))?;

    let mut query = parsed_url.query_pairs().into_owned();
    let ac_id = query
        .find(|(key, _)| key == "ac_id")
        .with_context(|| format!("failed to get ac_id from `{}`", redirect_url.underline()))?;
    Ok(ac_id.1)
}

/// SRUN portal response type when calling login/logout
///
/// Note that fields that are not used are omitted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrunPortalResponse {
    // present only when logging in and succeeds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suc_msg: Option<String>,

    // always present on logins and logouts
    pub client_ip: IpAddr,
    pub online_ip: IpAddr,
    pub error: String,
    pub error_msg: String,
    pub res: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SrunChallenge {
    // the only useful field that must be present
    pub challenge: String,
}

/// SRUN client
#[derive(Debug)]
pub struct SrunClient {
    // reusable http client
    pub http_client: Client,

    // srun login info, username is student id
    pub username: String,
    pub password: String,

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
        password: String,
        http_client: Option<Client>,
        ip: Option<IpAddr>,
    ) -> Result<SrunClient> {
        let http_client = http_client.unwrap_or_default();
        let ac_id = get_acid(&http_client).await?;
        let login_state = get_login_state(&http_client).await?;
        let ip = ip.unwrap_or(login_state.online_ip);
        Ok(SrunClient {
            http_client,
            username,
            password,
            ip,
            ac_id,
            login_state,
        })
    }

    /// Login to the SRUN portal
    pub async fn login(&self) -> Result<SrunPortalResponse> {
        // check if already logged in
        if self.login_state.error == "ok" {
            bail!(
                "{} already logged in",
                self.login_state.online_ip.to_string().underline()
            )
        }

        // construct checksum and crypto encodings
        let token = self.get_challenge().await?;

        let chksum_data = json!({
            "username": self.username.clone(),
            "password": self.password.clone(),
            "ip": self.ip.to_string(),
            "acid": self.ac_id.clone(),
            "enc_ver": String::from("srun_bx1"),
        });

        let json_chksum_data = serde_json::to_string(&chksum_data)?;
        let encoded_data = xencode(json_chksum_data.as_str(), token.as_str());
        let info = format!("{}{}", "{SRBX1}", fkbase64(encoded_data));

        // construct param payload
        let mac = Hmac::<Md5>::new_from_slice(token.as_bytes())?;
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
            .with_context(|| "failed to send request when logging in")?;
        let raw_text = resp.text().await?;

        if raw_text.len() < 8 {
            bail!("login response too short: `{}`", raw_text)
        }
        let raw_json = &raw_text[6..raw_text.len() - 1];
        serde_json::from_str::<SrunPortalResponse>(raw_json)
            .with_context(|| format!("failed to parse malformed login response:\n  {}", raw_json))
    }

    /// Logout of the SRUN portal
    pub async fn logout(&self) -> Result<SrunPortalResponse> {
        // check if already logged out
        if self.login_state.error == "not_online_error" {
            bail!("{} already logged out", self.ip.to_string().underline())
        }

        // check if username match
        let logged_in_username = self.login_state.user_name.clone().unwrap_or_default();
        if logged_in_username != self.username {
            println!(
                "{} logged in user {} does not match yourself {}",
                "warning:".yellow(),
                format!("({})", logged_in_username).dimmed(),
                format!("({})", self.username).dimmed()
            );

            // tip to provide user override
            println!(
                "{:>8} provide username argument {} to override and logout current session",
                "tip:".cyan(),
                format!("`--user {}`", logged_in_username).bold().green()
            )
        }

        // check if ip match
        let logged_in_ip = self.login_state.online_ip;
        if logged_in_ip != self.ip {
            println!(
                "{} logged in ip (`{}`) does not match `{}`",
                "warning:".yellow(),
                logged_in_ip.to_string().underline(),
                self.ip.to_string().underline()
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
            .with_context(|| "failed to send request when logging out")?;
        let raw_text = resp.text().await?;
        let raw_json = &raw_text[6..raw_text.len() - 1];
        serde_json::from_str::<SrunPortalResponse>(raw_json)
            .with_context(|| format!("failed to parse malformed logout response:\n  {}", raw_json))
    }

    async fn get_challenge(&self) -> Result<String> {
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
            .with_context(|| "failed to get challenge")?;
        let raw_text = resp.text().await?;

        if raw_text.len() < 8 {
            bail!("logout response too short: `{}`", raw_text)
        }
        let raw_json = &raw_text[6..raw_text.len() - 1];
        let parsed_json = serde_json::from_str::<SrunChallenge>(raw_json).with_context(|| {
            format!(
                "failed to parse malformed get_challenge response:\n  {}",
                raw_json
            )
        })?;
        Ok(parsed_json.challenge)
    }
}
