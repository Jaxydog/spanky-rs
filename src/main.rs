#![deny(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
#![warn(clippy::nursery, clippy::pedantic, clippy::try_err)]
#![warn(clippy::todo, clippy::unimplemented)]
#![allow(clippy::module_name_repetitions, clippy::unused_async)]
#![allow(clippy::wildcard_imports)]
#![feature(is_some_and, const_trait_impl)]

use clap::Parser;
use prelude::*;

mod command;
mod prelude;
mod utility;

/// Open source guild harrassment bot
#[derive(Debug, Parser)]
#[command(author, about, long_about, version)]
struct Args {
    /// Disable logging
    #[arg(long, short)]
    quiet: bool,
    /// Disable log storage
    #[arg(long)]
    no_store: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args { no_store, quiet } = Args::parse();
    dotenvy::dotenv()?;

    let logger = Logger::new(quiet, !no_store)?;
    logger.info("Starting...")?;

    let mut client = Client::builder(token()?, INTENTS)
        .event_handler(Events::new(logger))
        .await?;

    client.start_autosharded().await.map_err(Into::into)
}
