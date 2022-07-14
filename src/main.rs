mod alias;
mod args;
mod fetch;
mod units;

use args::{Commands, SwellionArgs};
use clap::Parser;
use std::error::Error;

// TODO: add persistence (https://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/9-use-default-journal)
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = SwellionArgs::parse();

    match &args.subcommand {
        Commands::Fetch(fetch) => {
            fetch.fetch().await?;
        }
        Commands::Alias(alias) => {
            alias.create();
        }
        Commands::Units(units) => {
            units.run_updater();
        }
    }

    Ok(())
}
