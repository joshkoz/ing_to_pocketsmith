use std::io::Write;
use std::{
    env,
    fs::{self, File},
};

use crate::{
    config::Config, pocketsmith::transaction_account::PocketSmithClient, prelude::*,
    transaction_csv_parser,
};

use chrono::NaiveDate;
use regex::Regex;
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

#[derive(Debug, Clone)]
struct PocketSmithTransactionSlim {
    pub payee: String,
    pub date: NaiveDate,
    pub amount: f64,
}

pub async fn run() -> Result<()> {
    let config = Config::build(env::args())?;

    let filename = "config.toml";

    let contents = fs::read_to_string(filename)?;

    let data: ConfigFile = toml::from_str(&contents).unwrap();
    // println!("{:?}", data);

    let file = File::open(&config.path)?;

    let hash_map = transaction_csv_parser::parse(file, usize::MAX)?;
    // let file = File::create("debug_hashmap.txt").unwrap();
    // write!(&file, "{:#?}", hash_map);
    let candidate_count: usize = hash_map.values().map(|x| x.len()).sum();
    println!("{} candidate transactions to process", candidate_count);

    const PAGES_COUNT_TO_FETCH: usize = 400;
    // NOTE: Only a maximum of 100 per page is allowed.
    const PER_PAGE: usize = 100;

    let pocketsmith_client = PocketSmithClient::new(data.pocketsmith.developer_key);

    let re = Regex::new(r" +").unwrap();

    // Loop through all the account ids we have.
    for ing_account_id in hash_map.keys() {
        // Get the pocketsmith account id from the ing account id.

        let account_mapping = data
            .account
            .iter()
            .find(|a| &**ing_account_id == a.ing.as_str());
        if account_mapping.is_none() {
            println!("WARN: Couldn't find a pocketsmith account mapping for the ing account {ing_account_id}");
            continue;
        }
        let pocketsmith_account_id = account_mapping.unwrap().pocketsmith.as_str();

        let mut pocketsmith_transactions = Vec::new();
        // Loop through a bunch of pages for the current accout and append the transactions for
        // that account to the transaction list..
        println!(
            "\nFetching pocketsmith transactions for account mapping {} <-> {}",
            ing_account_id, pocketsmith_account_id
        );
        for i in 1..=PAGES_COUNT_TO_FETCH {
            // println!(
            //     "DEBUG --> {}: Fetching page {}/{}",
            //     pocketsmith_account_id, i, PAGES_COUNT_TO_FETCH
            // );
            let mut response = pocketsmith_client
                .list_transactions(
                    pocketsmith_account_id,
                    vec![("page", i.to_string()), ("per_page", PER_PAGE.to_string())],
                )
                .await;
            if response.is_ok() {
                let mut transactions = response
                    .unwrap()
                    .into_iter()
                    .map(|x| PocketSmithTransactionSlim {
                        date: NaiveDate::parse_from_str(&x.date, "%Y-%m-%d").unwrap(),
                        payee: re.replace_all(&x.payee, " ").to_string(),
                        amount: x.amount,
                    })
                    .collect();
                pocketsmith_transactions.append(&mut transactions);
            } else {
                dbg!(response);
                break;
            }
        }
        println!("Fetched {} transactions", pocketsmith_transactions.len());
        // let file = File::create(format!("pocketsmith-tx-{ing_account_id}.txt")).unwrap();
        // write!(&file, "{:#?}", pocketsmith_transactions);

        // Loop through the each transaction in the hashmap for this account.
        let ing_transactions = hash_map.get(ing_account_id).unwrap();

        for candidate_transaction in ing_transactions {
            // filter the transactions to ones that occured on the same date that have the same
            // amount
            let mut filtered_list_a = pocketsmith_transactions
                .clone()
                .into_iter()
                .filter(|t| t.date == candidate_transaction.date)
                .collect::<Vec<_>>();
            let mut filtered_list_b = filtered_list_a
                .clone()
                .into_iter()
                .filter(|t| t.amount == candidate_transaction.amount)
                .collect::<Vec<_>>();
            let is_duplicate = filtered_list_b
                .iter()
                .any(|t| t.payee == candidate_transaction.payee);

            // If it's a duplicate skip it.
            // else create it.
            if is_duplicate {
                // println!(
                //     "Skipping duplicate transaction ({}): {} {}",
                //     ing_account_id, candidate_transaction.date, candidate_transaction.desc
                // );
            } else {
                println!(
                    "Creating transaction ({}): {} {}",
                    ing_account_id, candidate_transaction.date, candidate_transaction.payee
                );
                pocketsmith_client
                    .create_transaction(pocketsmith_account_id, candidate_transaction)
                    .await
                    .unwrap();
                // dbg!(candidate_transaction);
                // dbg!(filtered_list_a);
                // dbg!(filtered_list_b);
                //
            }
        }
    }

    Ok(())
}
