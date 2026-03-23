use crate::crypto::{build_client_id, generate_pkce_pair};
use crate::Result;
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Write;
use tracing::{debug, info, warn};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthInfo {
    pub access_token: String,
    pub refresh_token: String,
    pub expires: u64,
    pub adp_token: String,
    pub device_private_key: String,
}

#[derive(Debug, Deserialize)]
struct RefreshResponse {
    access_token: String,
    expires_in: u64,
}

impl AuthInfo {
    /// Returns true if the access token is expired or expires in less than 60 seconds
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp() as u64;
        self.expires <= now + 60
    }

    /// Refreshes the access token via Amazon's OAuth endpoint
    pub async fn refresh_access_token(&mut self) -> Result<()> {
        info!("Refreshing Audible access token...");
        let http = Client::new();

        let params = [
            ("app_name", "Audible"),
            ("app_version", "3.56.2"),
            ("source_token", &self.refresh_token),
            ("requested_token_type", "access_token"),
            ("source_token_type", "refresh_token"),
        ];

        let resp = http
            .post("https://api.amazon.com/auth/token")
            .form(&params)
            .send()
            .await?
            .error_for_status()?;

        let data: RefreshResponse = resp.json().await?;

        self.access_token = data.access_token;
        self.expires = (Utc::now().timestamp() as u64) + data.expires_in;

        debug!(
            "Access token refreshed successfully. Expires in {}s",
            data.expires_in
        );
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct RegisterResponse {
    response: RegisterSuccess,
}

#[derive(Debug, Deserialize)]
struct RegisterSuccess {
    success: RegisterTokens,
}

#[derive(Debug, Deserialize)]
struct RegisterTokens {
    tokens: TokensBlock,
}

#[derive(Debug, Deserialize)]
struct TokensBlock {
    bearer: BearerToken,
    mac_dms: MacDmsToken,
}

#[derive(Debug, Deserialize)]
struct BearerToken {
    access_token: String,
    refresh_token: String,
    expires_in: String,
}

#[derive(Debug, Deserialize)]
struct MacDmsToken {
    adp_token: String,
    device_private_key: String,
}

/// Generates the Amazon OAuth login URL, waits on a local HTTP server for the redirect callback,
/// and exchanges the authorization_code for Audible Device keys natively.
pub async fn login_with_browser() -> Result<AuthInfo> {
    debug!("Initializing browser login flow...");

    // 1. Generate PKCE & Serial
    let (verifier, challenge) = generate_pkce_pair();
    let serial = uuid::Uuid::new_v4().as_simple().to_string().to_uppercase();
    let country_code = "us"; // We can make this configurable later
    let domain = "com";
    let market_place_id = "ATVPDKIKX0DER"; // US marketplace

    let login_url = format!(
        "https://www.amazon.{}/ap/signin?openid.oa2.response_type=code&openid.oa2.code_challenge_method=S256&openid.oa2.code_challenge={}&openid.return_to=https://www.amazon.{}/ap/maplanding&openid.assoc_handle=amzn_audible_ios_{}&openid.identity=http://specs.openid.net/auth/2.0/identifier_select&pageId=amzn_audible_ios&accountStatusPolicy=P1&openid.claimed_id=http://specs.openid.net/auth/2.0/identifier_select&openid.mode=checkid_setup&openid.ns.oa2=http://www.amazon.com/ap/ext/oauth/2&openid.oa2.client_id=device:{}&openid.ns.pape=http://specs.openid.net/extensions/pape/1.0&marketPlaceId={}&openid.oa2.scope=device_auth_access&forceMobileLayout=true&openid.ns=http://specs.openid.net/auth/2.0&openid.pape.max_auth_age=0",
        domain, challenge, domain, country_code, build_client_id(&serial), market_place_id
    );

    println!("===========================================================");
    println!("Please open the following URL in your web browser to login:\n");
    println!("{}\n", login_url);
    println!("Note: After logging in, the browser will likely show a 'Page not found' error.");
    println!("Please copy the ENTIRE URL from your address bar and paste it below.");
    println!("===========================================================\n");

    // Wait for the user to paste the callback URL instead of running a local server.
    // Amazon's new /ap/maplanding doesn't redirect to localhost, it just stays on maplanding.
    // The user MUST copy the URL from their browser's address bar.
    let mut input = String::new();
    print!("Paste the result URL here: ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    let url_str = input.trim();

    let auth_code = extract_code_from_url(url_str).ok_or_else(|| {
        crate::Error::Auth(
            "Failed to find openid.oa2.authorization_code in the provided URL".to_string(),
        )
    })?;

    info!("Exchanging auth code for device registration tokens...");

    // 2. Exchange code for device registration tokens
    let http = Client::new();
    let api_url = format!("https://api.amazon.{}/auth/register", domain);

    let payload = json!({
        "requested_token_type": [
            "bearer",
            "mac_dms"
        ],
        "cookies": {
            "website_cookies": [],
            "domain": format!(".amazon.{}", domain)
        },
        "registration_data": {
            "domain": "Device",
            "app_version": "3.56.2",
            "device_serial": serial,
            "device_type": "A2CZJZGLK2JJVM",
            "device_name": "%FIRST_NAME%%FIRST_NAME_POSSESSIVE_STRING%%DUPE_STRATEGY_1ST%Audible for iPhone",
            "os_version": "15.0.0",
            "software_version": "35602678",
            "device_model": "iPhone",
            "app_name": "Audible"
        },
        "auth_data": {
            "client_id": build_client_id(&serial),
            "authorization_code": auth_code,
            "code_verifier": verifier,
            "code_algorithm": "SHA-256",
            "client_domain": "DeviceLegacy"
        },
        "requested_extensions": ["device_info", "customer_info"]
    });

    let resp = http.post(&api_url).json(&payload).send().await?;

    if !resp.status().is_success() {
        let status = resp.status().to_string();
        let body = resp.text().await.unwrap_or_default();
        warn!("Amazon registration failed: {} - {}", status, body);
        return Err(crate::Error::RegisterError { status, body });
    }

    let data: RegisterResponse = resp.json().await?;
    let tokens = data.response.success.tokens;

    let expires_in: u64 = tokens.bearer.expires_in.parse().unwrap_or(3600);
    let expires = (Utc::now().timestamp() as u64) + expires_in;

    Ok(AuthInfo {
        access_token: tokens.bearer.access_token,
        refresh_token: tokens.bearer.refresh_token,
        expires,
        adp_token: tokens.mac_dms.adp_token,
        device_private_key: tokens.mac_dms.device_private_key,
    })
}

fn extract_code_from_url(url: &str) -> Option<String> {
    // Basic parsing to extract `openid.oa2.authorization_code=...`
    let parts: Vec<&str> = url.split('?').collect();
    if parts.len() < 2 {
        return None;
    }
    let query = parts[1];
    for pair in query.split('&') {
        let kv: Vec<&str> = pair.split('=').collect();
        if kv.len() == 2 && kv[0] == "openid.oa2.authorization_code" {
            // Unescape in case it was url-encoded, though typically the code is safe chars
            return Some(kv[1].to_string());
        }
    }
    None
}
