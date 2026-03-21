use crate::cli::Commands;

use std::path::PathBuf;

fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("audiobook-downloader");
    std::fs::create_dir_all(&path).unwrap();
    path
}

pub async fn handle(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Auth => auth().await,
        Commands::Sync => sync().await,
        Commands::List => list().await,
        Commands::Info { id } => info(&id).await,
        Commands::Download { id, all } => download(id.as_deref(), all).await,
        Commands::Config => config().await,
    }
}

async fn auth() -> anyhow::Result<()> {
    println!("Starting native Audible authentication flow...");
    let auth_info = audible_api::auth::login_with_browser().await?;
    
    let out_json = serde_json::to_string_pretty(&auth_info)?;
    let auth_path = get_config_dir().join("auth.json");
    std::fs::write(&auth_path, out_json)?;
    
    println!("Successfully authenticated! Credentials saved natively to {}", auth_path.display());
    Ok(())
}

async fn sync() -> anyhow::Result<()> {
    println!("Syncing library...");
    
    let auth_path = get_config_dir().join("auth.json");
    let token_data = std::fs::read_to_string(&auth_path)
        .map_err(|_| anyhow::anyhow!("Please run 'audiobook-downloader auth' first."))?;
        
    let mut auth: audible_api::auth::AuthInfo = serde_json::from_str(&token_data)?;

    if auth.is_expired() {
        println!("Access token expired. Refreshing...");
        auth.refresh_access_token().await?;
        
        let out_json = serde_json::to_string_pretty(&auth)?;
        std::fs::write(&auth_path, out_json)?;
        println!("Refreshed successfully!");
    }

    let client = audible_api::Client::new(auth);
    
    println!("Fetching activation bytes...");
    let act_bytes = client.get_activation_bytes().await?;
    println!("Activation Bytes: {}", act_bytes);

    println!("Fetching library...");
    let library = client.get_library().await?;
    println!("Found {} books. Displaying first 3:", library.len());
    
    for book in library.iter().take(3) {
        println!("- {} ({})", book.title, book.asin);
    }
    
    if let Some(first_book) = library.first() {
        println!("Fetching download URL for {}...", first_book.title);
        let url = client.get_aax_download_url(&first_book.asin).await;
        match url {
            Ok(u) => println!("Download URL: {}", u),
            Err(e) => println!("Could not fetch download URL: {}", e),
        }
    }

    Ok(())
}

async fn list() -> anyhow::Result<()> {
    println!("Listing locally cached books...");
    Ok(())
}

async fn info(id: &str) -> anyhow::Result<()> {
    println!("Info for book {}", id);
    Ok(())
}

async fn download(id: Option<&str>, all: bool) -> anyhow::Result<()> {
    if all {
        println!("Downloading all missing books...");
    } else if let Some(book_id) = id {
        println!("Downloading book {}", book_id);
    }
    Ok(())
}

async fn config() -> anyhow::Result<()> {
    println!("Config management...");
    Ok(())
}
