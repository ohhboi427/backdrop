mod auth;
mod error;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Auth,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use crate::auth::auth;
    use keyring::Entry;
    use unsplash_api::auth::AuthToken;
    use unsplash_api::client::Client;

    let token_entry = Entry::new("Backdrop", "bearer_token")?;

    let cli = Cli::parse();
    let token = match cli.command {
        Some(Commands::Auth) => {
            let token = auth().await?;
            token_entry.set_password(token.as_ref())?;

            token
        }
        None => match token_entry.get_password() {
            Ok(token) => AuthToken::Bearer(token),
            Err(keyring::Error::NoEntry) => {
                let token = auth().await?;
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
