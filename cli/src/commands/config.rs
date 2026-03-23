use crate::error::Result;
use tracing::info;

pub async fn config() -> Result<()> {
    info!("Managing configuration...");
    println!("Config management...");
    Ok(())
}
