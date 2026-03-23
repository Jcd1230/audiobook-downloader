use crate::config::Config;
use crate::error::Result;
use tracing::info;

pub async fn import(_config: Config) -> Result<()> {
    info!("Importing library...");
    Ok(())
}
