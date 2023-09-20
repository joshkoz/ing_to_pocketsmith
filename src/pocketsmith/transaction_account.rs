use reqwest::{
    header::{self, HeaderValue},
    Client, Url,
};

use super::transaction::Transaction;

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
    pub async fn get_user(&self) -> anyhow::Result<String> {
        let url = Url::parse(&self.base_url)?;
        let url = url.join(&format!("{}", "me")).unwrap();
        let request = self.http_client.get(url);
        let response = request.send().await?.text().await?;

        Ok(response)
    }

    pub async fn list_transactions(
        &self,
        id: impl AsRef<str>,
        params: Vec<(&'static str, String)>,
    ) -> anyhow::Result<Vec<Transaction>> {
        let url = Url::parse(&self.base_url)?;
        let url = url
            .join(&format!(
                "{}{}{}",
                "./transaction_accounts/",
                id.as_ref(),
                "/transactions"
            ))
            .unwrap();
        let request = self.http_client.get(url).query(&params);
        let response: Vec<Transaction> = request.send().await?.json().await?;

        Ok(response)
    }

    pub async fn find_transaction_account(&self, id: impl AsRef<str>) -> anyhow::Result<String> {
        let url = Url::parse(&self.base_url)?;
        let url = url
            .join(&format!("{}{}", "./transaction_accounts/", id.as_ref()))
            .unwrap();
        let request = self.http_client.get(url);
        let response = request.send().await?.text().await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn build_transactions() {
        const PAGES_COUNT_TO_FETCH: usize = 10;
        // NOTE: Only a maximum of 100 per page is allowed.
        const PER_PAGE: usize = 100;
        let p = PocketSmithClient::new("".to_string());
        let mut transaction_list = Vec::new();
        for i in 1..=PAGES_COUNT_TO_FETCH {
            let mut response = p
                .list_transactions(
                    "819702",
                    vec![("page", i.to_string()), ("per_page", PER_PAGE.to_string())],
                )
                .await;
            if response.is_ok() {
                transaction_list.append(&mut response.unwrap());
            } else {
                dbg!(response);
                break;
            }
        }
        assert_eq!(PER_PAGE * PAGES_COUNT_TO_FETCH, transaction_list.len())
    }

    #[tokio::test]
    async fn list_transactions() {
        let p = PocketSmithClient::new("".to_string());
        let response = p
            .list_transactions("819702", vec![("page", "1".to_string())])
            .await;

        assert_eq!(dbg!(response).unwrap()[0].id, 453351987);
    }

    #[tokio::test]
    async fn find_me() {
        let p = PocketSmithClient::new("".to_string());
        let response = p.get_user().await.unwrap();
        assert_eq!(response, "hello");
    }

    #[tokio::test]
    async fn check() {
        let p = PocketSmithClient::new("".to_string());
        let response = p.find_transaction_account("819702").await.unwrap();
        assert_eq!(response, "hello");
    }
}
