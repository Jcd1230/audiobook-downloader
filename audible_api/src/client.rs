use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use anyhow::Result;
use crate::auth::AuthInfo;

#[derive(Debug, Clone)]
pub struct Client {
    http: ReqwestClient,
    auth: AuthInfo,
}

impl Client {
    pub fn new(auth: AuthInfo) -> Self {
        Self {
            http: ReqwestClient::new(),
            auth,
        }
    }

    /// Helper to get the base API URL (e.g. https://api.audible.com/1.0)
    fn base_url() -> &'static str {
        "https://api.audible.com/1.0"
    }

    pub async fn get_library(&self) -> Result<Vec<LibraryItem>> {
        let url = format!("{}/library?response_groups=contributors,product_desc,media", Self::base_url());
        
        let mut req = self.http.get(&url).build()?;
        crate::crypto::sign_request(&mut req, &self.auth.adp_token, &self.auth.device_private_key)?;

        let response = self.http.execute(req).await?.text().await?;
        
        let library_response = serde_json::from_str::<LibraryResponse>(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse library: {}\nPayload: {}", e, response))?;
            
        Ok(library_response.items)
    }

    /// Requests the account's DRM activation bytes.
    pub async fn get_activation_bytes(&self) -> Result<String> {
        let url = "https://www.audible.com/license/token?action=register&player_manuf=Audible,iPhone&player_model=iPhone";
        
        for _ in 0..5 {
            let mut req = self.http.get(url).build()?;
            crate::crypto::sign_request(&mut req, &self.auth.adp_token, &self.auth.device_private_key)?;

            let response_bytes = self.http.execute(req).await?.bytes().await?;
            let blob = response_bytes.to_vec();

            if blob.len() >= 568 {
                let data = &blob[blob.len() - 568..];
                
                let mut joined_data = Vec::with_capacity(70 * 8);
                for i in 0..8 {
                    let start = i * 71;
                    if start + 70 <= data.len() {
                        joined_data.extend_from_slice(&data[start..start + 70]);
                    }
                }
                
                if joined_data.len() >= 4 {
                    let act_bytes_slice = &joined_data[0..4];
                    let act_bytes = u32::from_le_bytes(act_bytes_slice.try_into()?);
                    return Ok(format!("{:08x}", act_bytes));
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        
        anyhow::bail!("Failed to acquire a valid activation blob after retries.")
    }

    /// Fetches the download URL for an audiobook using the official licenserequest endpoint.
    pub async fn get_aax_download_url(&self, asin: &str) -> Result<String> {
        let url = format!("{}/content/{}/licenserequest", Self::base_url(), asin);
        
        let body = serde_json::json!({
            "supported_drm_types": ["Adrm", "Mpeg"],
            "quality": "High",
            "consumption_type": "Download",
            "response_groups": "last_position_heard,pdf_url,content_reference"
        });

        let mut req = self.http.post(&url)
            .header("X-ADP-SW", "37801821")
            .header("X-ADP-Transport", "WIFI")
            .header("X-ADP-LTO", "120")
            .header("X-Device-Type-Id", "A2CZJZGLK2JJVM")
            .header("device_idiom", "phone")
            .json(&body)
            .build()?;
            
        crate::crypto::sign_request(&mut req, &self.auth.adp_token, &self.auth.device_private_key)?;

        let response: serde_json::Value = self.http.execute(req).await?.json().await?;
        
        let download_url = response["content_license"]["content_metadata"]["content_url"]["offline_url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Could not find offline_url in license response: {}", response))?;
            
        Ok(download_url.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct LibraryItem {
    pub asin: String,
    pub title: String,
    pub authors: Option<Vec<Contributor>>,
    pub narrators: Option<Vec<Contributor>>,
    pub runtime_length_min: Option<u64>,
    pub product_images: Option<ProductImages>,
    pub is_downloaded: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Contributor {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductImages {
    #[serde(rename = "500")]
    pub size_500: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryResponse {
    pub items: Vec<LibraryItem>,
}
