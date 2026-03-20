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
        
        let req = self.http.get(&url)
            .bearer_auth(&self.access_token)
            // Add required response_groups as defined in AudibleApi's README
            .query(&[("response_groups", "authors,available_codecs,category_ladders,content_delivery_type,content_type,format_type,has_children,is_adult_product,is_listenable,issue_date,language,merchandising_summary,narrators,product_images,publisher_name,release_date,runtime_length_min,subtitle,title,pdf_link")])
            .build()?;

        // This is a stubbed response for now to ensure type compilation
        println!("GET {}", url);
        
        // In a real implementation:
        // let response = self.http.execute(req).await?.json::<LibraryResponse>().await?;
        // Ok(response.items)

        Ok(vec![])
    }
}

#[derive(Debug, Deserialize)]
pub struct LibraryItem {
    pub asin: String,
    pub title: String,
    // Add authors, narrators, images, etc. based on AudibleApi/AudibleApi.Common/LibraryDtoV10.cs
}

#[derive(Debug, Deserialize)]
struct LibraryResponse {
    items: Vec<LibraryItem>,
}
