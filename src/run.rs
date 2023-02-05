use std::{env, fs::File};

use crate::{config::Config, prelude::*, transaction_csv_parser};

use phf::phf_map;

static ACCOUNT_MAP: phf::Map<&'static str, u64> = phf_map! {
    "38422180" => 819702,   // Everyday account
    "300010508" => 908078,  // Defunct income account
    "41279290" => 819721,   // Savings account
    "200050362" => 1355406, // Offset account mortgage account
    "200050373" => 1355409 // Fixed rate mortgage account
};

pub fn run() -> Result<()> {
    let config = Config::build(env::args())?;

    let file = File::open(&config.path)?;

    let hash_map = transaction_csv_parser::parse(file, usize::MAX)?;
    println!("{:#?}", hash_map);

    Ok(())
}
