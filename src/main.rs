#![allow(unused)]

use crate::prelude::*;

use anyhow::{Context, Result};
use dotenv::dotenv;
use ing_to_pocketsmith::config::Config;

mod error;
mod prelude;
mod transaction_csv_parser;

#[tokio::main]
async fn main() {
    dotenv().ok();

    if let Err(err) = ing_to_pocketsmith::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
