pub mod transaction;
pub mod transaction_account;

pub struct PocketSmith {
    developer_key: String,
    user_id: String,
}

impl PocketSmith {
    pub fn new(developer_key: String, pocket_smith_user_id: String) -> Self {
        Self {
            developer_key,
            user_id: pocket_smith_user_id,
        }
    }

    pub fn hello(&self) -> &'static str {
        "hello"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let p = PocketSmith::new(String::new(), String::new());
        let data = p.hello();

        assert_eq!("hello", data);
    }
}
