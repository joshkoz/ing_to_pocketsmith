use std::{
    env,
    fs::{self, File},
};

use crate::{config::Config, prelude::*, transaction_csv_parser};

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    account: Vec<AccountMapingEntry>,
    pocketsmith: PocketSmith,
}

#[derive(Debug, Deserialize)]
struct PocketSmith {
    developer_key: String,
    user_id: String,
}

#[derive(Debug, Deserialize)]
struct AccountMapingEntry {
    ing: String,
    pocketsmith: String,
}

pub fn run() -> Result<()> {
    let config = Config::build(env::args())?;

    let filename = "config.toml";

    let contents = fs::read_to_string(filename)?;

    let data: ConfigFile = toml::from_str(&contents).unwrap();
    println!("{:?}", data);

    let file = File::open(&config.path)?;

    let hash_map = transaction_csv_parser::parse(file, usize::MAX)?;
    println!("{:#?}", hash_map);

    Ok(())
}
