mod commands;
mod error;

use crate::commands::auth::AuthCommand;
use crate::error::Error;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Auth {
        #[clap(subcommand)]
        command: Option<AuthCommand>,
    },
    Fetch,
}

impl Command {
    async fn run(self) -> Result<(), Error> {
        match self {
            Self::Auth { command } => {
                command.unwrap_or_default().run().await?;
            }

            Self::Fetch => {
                use crate::commands::auth::ensure_auth_token;
                use unsplash_api::client::Client;

                let token = ensure_auth_token().await?;
                println!("{}", token);

                let _client = Client::new(&token);
            }
        }

        Ok(())
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::Fetch
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.command.unwrap_or_default().run().await?;

    Ok(())
}
