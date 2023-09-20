use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub payee: String,
    original_payee: String,
    pub date: String,
    upload_source: String,
    category: Category,
    closing_balance: f64,
    cheque_number: Option<String>,
    memo: Option<String>,
    pub amount: f64,
    amount_in_base_currency: f64,
    #[serde(rename = "type")]
    transaction_type: String,
    is_transfer: Option<bool>,
    needs_review: bool,
    status: String,
    note: Option<String>,
    labels: Vec<String>,
    transaction_account: TransactionAccount,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    id: u64,
    title: String,
    colour: Option<String>,
    is_transfer: bool,
    is_bill: bool,
    refund_behaviour: Option<String>,
    // children: Vec<String>, // We don't need this.
    parent_id: Option<u64>,
    roll_up: bool,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionAccount {
    id: u64,
    account_id: u64,
    name: String,
    latest_feed_name: Option<String>,
    number: Option<String>,
    #[serde(rename = "type")]
    account_type: String,
    offline: bool,
    is_net_worth: bool,
    currency_code: String,
    current_balance: f64,
    current_balance_in_base_currency: f64,
    current_balance_exchange_rate: Option<f64>,
    current_balance_date: String,
    current_balance_source: String,
    data_feeds_balance_type: String,
    safe_balance: Option<f64>,
    safe_balance_in_base_currency: Option<f64>,
    has_safe_balance_adjustment: bool,
    starting_balance: f64,
    starting_balance_date: String,
    institution: Institution,
    data_feeds_account_id: Option<u64>,
    data_feeds_connection_id: Option<u64>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Institution {
    id: u64,
    title: String,
    currency_code: String,
    created_at: String,
    updated_at: String,
}
