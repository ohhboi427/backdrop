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
    use crate::commands::auth::*;
    use keyring::Entry;
    use unsplash_api::auth::AuthToken;
    use unsplash_api::client::Client;

    let token_entry = Entry::new("Backdrop", "bearer_token")?;

    let cli = Cli::parse();
    let token = match cli.command {
        Some(Commands::Auth {
            command: Some(AuthCommand::Remove),
        }) => {
            token_entry.delete_credential()?;

            return Ok(());
        }
        Some(Commands::Auth { command: None }) => {
            let token = obtain_auth_token().await?;
            token_entry.set_password(token.as_ref())?;

            return Ok(());
        }
        None => match token_entry.get_password() {
            Ok(token) => AuthToken::Bearer(token),
            Err(keyring::Error::NoEntry) => {
                let token = obtain_auth_token().await?;
                token_entry.set_password(token.as_ref())?;

                token
            }
            Err(e) => return Err(e.into()),
        },
    };

    println!("{}", token);
    let _client = Client::new(&token);

    Ok(())
}
