use reqwest::{
    header::{self, HeaderValue},
    Client, Url,
};

pub struct TransactionAccount {
    id: String,
    name: String,
    number: String,
    _type: String,
    currency_code: String,
    current_balance: String,
    current_balance_in_base_currency: String,
    current_balance_date: String,
    starting_balance: String,
    starting_balance_date: String,
    institution: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug)]
pub struct PocketSmithClient {
    http_client: Client,
    base_url: String,
}

impl PocketSmithClient {
    pub fn new(developer_key: String) -> Self {
        let mut headers = header::HeaderMap::new();

        let mut auth_value = header::HeaderValue::from_str(&developer_key).unwrap();
        auth_value.set_sensitive(true);

        headers.insert("X-Developer-Key", auth_value);
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self {
            http_client: client,
            base_url: "https://api.pocketsmith.com/v2/".to_owned(),
        }
    }

    pub async fn find_transaction_account(&self, id: impl AsRef<str>) -> anyhow::Result<String> {
        let url = Url::parse(&self.base_url)?;
        let url = url
            .join(&format!("{}{}", "./transaction_accounts/", id.as_ref()))
            .unwrap();
        let request = dbg!(self.http_client.get(url));
        let response = dbg!(request.send().await?.text().await?);

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check() {
        let p = PocketSmithClient::new(String::new());
        let response = p.find_transaction_account("161369").await.unwrap();
        assert_eq!(response, "hello");
    }
}
