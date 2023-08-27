#![allow(unused)]

use crate::prelude::*;

use anyhow::{Context, Result};
use pocketsmith_importer::config::Config;

mod error;
mod prelude;
mod transaction_csv_parser;

#[tokio::main]

async fn main() {
    if let Err(err) = pocketsmith_importer::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
