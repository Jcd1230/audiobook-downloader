use crate::cli::Commands;

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
    println!("Auth flow...");
    Ok(())
}

async fn sync() -> anyhow::Result<()> {
    println!("Syncing library...");
    
    // For testing, we read the provided token directly
    let token_data = std::fs::read_to_string("../jcd1230@gmail.com.json")
        .or_else(|_| std::fs::read_to_string("jcd1230@gmail.com.json"))?;
        
    let parsed: serde_json::Value = serde_json::from_str(&token_data)?;
    let mut auth = audible_api::auth::AuthInfo {
        access_token: parsed["access_token"].as_str().unwrap_or("").to_string(),
        refresh_token: parsed["refresh_token"].as_str().unwrap_or("").to_string(),
        expires: parsed["expires"].as_u64().unwrap_or(0),
        adp_token: parsed["adp_token"].as_str().unwrap_or("").to_string(),
        device_private_key: parsed["device_private_key"].as_str().unwrap_or("").to_string(),
    };

    if auth.is_expired() {
        println!("Access token expired. Refreshing...");
        auth.refresh_access_token().await?;
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
