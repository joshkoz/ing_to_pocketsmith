use std::{env, fs::File};

use crate::{config::Config, prelude::*, transaction_csv_parser};

pub fn run() -> Result<()> {
    let config = Config::build(env::args())?;

    let file = File::open(&config.path)?;

    let hash_map = transaction_csv_parser::parse(file)?;
    println!("{:#?}", hash_map);

    Ok(())
}
