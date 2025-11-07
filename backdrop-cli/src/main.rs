mod commands;
mod error;

use crate::commands::auth::AuthCommand;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Auth {
        #[clap(subcommand)]
        command: Option<AuthCommand>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use crate::commands::*;
    use unsplash_api::client::Client;

    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Auth {
                command: Some(AuthCommand::Remove),
            } => {
                delete_stored_auth_token()?;
            }
            Commands::Auth { command: None } => {
                obtain_and_store_auth_token().await?;
            }
        }

        return Ok(());
    }

    let token = match get_stored_auth_token()? {
        Some(token) => token,
        None => obtain_and_store_auth_token().await?,
    };

    println!("{}", token);
    let _client = Client::new(&token);

    Ok(())
}
