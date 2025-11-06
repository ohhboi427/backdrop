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
    let token = match cli.command {
        Some(Commands::Auth {
            command: Some(AuthCommand::Remove),
        }) => {
            delete_stored_auth_token()?;

            return Ok(());
        }
        Some(Commands::Auth { command: None }) => {
            obtain_and_store_auth_token().await?;

            return Ok(());
        }
        None => match get_stored_auth_token()? {
            Some(token) => token,
            None => obtain_and_store_auth_token().await?,
        },
    };

    println!("{}", token);
    let _client = Client::new(&token);

    Ok(())
}
