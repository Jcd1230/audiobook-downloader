use crate::cli::Cli;
use crate::error::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

pub async fn completions(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
    Ok(())
}
