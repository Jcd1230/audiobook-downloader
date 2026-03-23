use crate::commands::utils::get_config_dir;
use crate::error::Result;
use tracing::info;

pub async fn auth() -> Result<()> {
    info!("Starting native Audible authentication flow...");
    let auth_info = audible_api::auth::login_with_browser().await?;
    
    let out_json = serde_json::to_string_pretty(&auth_info).map_err(|e| anyhow::anyhow!(e))?;
    let auth_path = get_config_dir().join("auth.json");
    std::fs::write(&auth_path, out_json)?;
    
    println!("Successfully authenticated! Credentials saved natively to {}", auth_path.display());
    Ok(())
}
