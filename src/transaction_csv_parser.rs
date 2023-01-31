use serde_derive::Deserialize;
use std::{collections::HashMap, fs::File};

use crate::prelude::*;

#[derive(Debug, Deserialize, Clone)]
struct Row {
    #[serde(alias = "Date")]
    date: String,
    #[serde(alias = "Account")]
    account: Option<String>,
    #[serde(alias = "Description")]
    description: Option<String>,
    #[serde(alias = "Credit")]
    credit: Option<f64>,
    #[serde(alias = "Debit")]
    debit: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub date: String,
    pub description: String,
    pub amount: f64,
}

pub fn parse(file: File) -> Result<HashMap<String, Vec<Transaction>>> {
    let mut rdr = csv::ReaderBuilder::new()
        // .has_headers(false)
        .from_reader(&file);

    let csv_data: csv::DeserializeRecordsIter<&File, Row> = rdr.deserialize();

    let mut hash_map: HashMap<String, Vec<Transaction>> = HashMap::new();

    for record in csv_data {
        if let Ok(record) = record {
            let account_number = record
                .account
                .ok_or_else(|| Error::Generic(String::from("Account number is for row")))?;

            let vec = hash_map.entry(account_number.clone()).or_insert(Vec::new());

            let amount = record.debit.or(record.credit).unwrap_or(0f64);

            vec.push(Transaction {
                date: record.date,
                description: record.description.unwrap_or(String::new()),
                amount: amount,
            });
        }
    }

    Ok(hash_map)
}
