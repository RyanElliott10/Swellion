use clap::{Parser, Subcommand};

use crate::{alias::Alias, fetch::Fetch, units::Units};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct SwellionArgs {
    /// Sets an alias for lat/long coordinates
    #[clap(subcommand)]
    pub subcommand: Commands,

    /// Displays current unit settings
    #[clap(short, long)]
    pub units: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch data for a specified latitude, longitude
    Fetch(Fetch),

    /// Set an alias for a latitude, longitude
    Alias(Alias),

    /// Configure the units of measurement
    Units(Units)
}
