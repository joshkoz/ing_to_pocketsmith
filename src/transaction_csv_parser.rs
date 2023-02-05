use chrono::{DateTime, NaiveDate};
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
    pub date: NaiveDate,
    pub note: Option<String>,
    pub amount: f64,
    pub is_transfer: bool,
    pub merchant: String,
}

impl Transaction {
    fn try_build(row: Row) -> Result<Self> {
        let date: NaiveDate = NaiveDate::parse_from_str(&row.date, "%d/%m/%Y")?;
        let description = row
            .description
            .expect("Every transaction should have a description");
        let mut split = description.splitn(2, "-");
        let merchant = split.next().map(|s| s.trim().to_owned()).unwrap();
        let note = split.next().map(|s| s.trim().to_owned());
        let amount = row.debit.or(row.credit).unwrap_or(64 as f64);
        Ok(Transaction {
            date,
            amount,
            merchant,
            note,
            is_transfer: false,
        })
    }
}

pub fn parse(file: File, limit: usize) -> Result<HashMap<String, Vec<Transaction>>> {
    let mut rdr = csv::ReaderBuilder::new()
        // .has_headers(false)
        .from_reader(&file);

    let csv_data: csv::DeserializeRecordsIter<&File, Row> = rdr.deserialize();

    let mut hash_map: HashMap<String, Vec<Transaction>> = HashMap::new();

    let mut count = 0;
    for record in csv_data {
        if (count == limit) {
            return Ok(hash_map);
        }
        count += 1;
        if let Ok(mut record) = record {
            let account_number = match record.account.take() {
                Some(acc_number) => acc_number,
                None => return Err(Error::Generic(String::from("Account number is for row"))),
            };
            let vec = hash_map.entry(account_number).or_insert(Vec::new());
            vec.push(Transaction::try_build(record)?);
        }
    }

    Ok(hash_map)
}
