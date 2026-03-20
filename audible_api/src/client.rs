use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Client {
    http: ReqwestClient,
    access_token: String,
}

impl Client {
    pub fn new(access_token: String) -> Self {
        Self {
            http: ReqwestClient::new(),
            access_token,
        }
    }

    /// Helper to get the base API URL (e.g. https://api.audible.com/1.0)
    fn base_url() -> &'static str {
        "https://api.audible.com/1.0"
    }

    /// Fetches the user's library items
    pub async fn get_library(&self) -> Result<Vec<LibraryItem>> {
        let url = format!("{}/library", Self::base_url());
        
        // Using an empty/default response group to get the rich payload
        let req = self.http.get(&url)
            .bearer_auth(&self.access_token)
            .build()?;

        let response = self.http.execute(req).await?.json::<LibraryResponse>().await?;
        Ok(response.items)
    }

    /// Requests the account's DRM activation bytes.
    pub async fn get_activation_bytes(&self) -> Result<String> {
        let url = "https://www.audible.com/license/token?action=register&player_manuf=Audible,Android&player_model=Android";
        
        let req = self.http.get(url)
            .bearer_auth(&self.access_token)
            .build()?;

        let response = self.http.execute(req).await?.bytes().await?;
        
        // C# Libation sets ACTIVATION_BLOB_SZ = 0x238
        let blob_size = 568; 
        if response.len() < blob_size {
            anyhow::bail!("Activation blob is too small: {}", response.len());
        }
        
        let offset = response.len() - blob_size;
        let act_bytes_slice = &response[offset..offset + 4];
        
        let act_bytes = u32::from_le_bytes(act_bytes_slice.try_into()?);
        Ok(format!("{:08x}", act_bytes))
    }

    /// Fetches the download URL for an AAX file (the AAX workaround method).
    pub async fn get_aax_download_url(&self, asin: &str) -> Result<String> {
        // We use Amazon's CDE delivery service instead of api.audible.com
        let url = format!("https://cde-ta-g7g.amazon.com/FionaCDEServiceEngine/FSDownloadContent?type=AUDI&currentTransportMethod=WIFI&key={}&codec=aax", asin);
        
        // This request replies with a 302 Found redirect
        let redirect_client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
            
        let req = redirect_client.get(&url)
            .bearer_auth(&self.access_token)
            .build()?;

        let response = redirect_client.execute(req).await?;
        
        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                return Ok(location.to_str()?.to_string());
            }
        }
        
        anyhow::bail!("Failed to get download URL, expected 302 Redirect.")
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
