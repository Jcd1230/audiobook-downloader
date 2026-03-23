use crate::cli::ConfigSubcommand;
use crate::config::Config;
use crate::error::Result;
use tracing::info;

pub async fn config(subcommand: ConfigSubcommand, mut config: Config) -> Result<()> {
    match subcommand {
        ConfigSubcommand::Show => {
            println!("Current Configuration:");
            println!(
                "  library_path: {}",
                config.library_path.as_deref().unwrap_or("[Not Set]")
            );
            println!(
                "  filename_template: {}",
                config.filename_template.as_deref().unwrap_or("[Not Set]")
            );
        }
        ConfigSubcommand::Set { key, value } => {
            match key.as_str() {
                "library_path" => {
                    config.library_path = Some(value.clone());
                    info!("Set library_path to: {}", value);
                }
                "filename_template" => {
                    config.filename_template = Some(value.clone());
                    info!("Set filename_template to: {}", value);
                }
                _ => {
                    return Err(crate::error::CLIError::Anyhow(anyhow::anyhow!("Unknown configuration key: {}. Valid keys: library_path, filename_template", key)));
                }
            }
            config.save().map_err(|e| anyhow::anyhow!(e))?;
            println!("Successfully set {} to: {}", key, value);
        }
    }
    Ok(())
}
