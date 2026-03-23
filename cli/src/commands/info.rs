use crate::error::Result;
use tracing::info;

pub async fn info(id: &str) -> Result<()> {
    info!("Fetching info for book {}", id);
    println!("Info for book {}", id);
    Ok(())
}
