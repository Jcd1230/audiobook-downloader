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
