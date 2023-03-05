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

    fn setup() {
        dotenv::dotenv().ok();
    }

    #[test]
    fn check() {
        setup();
        let p = PocketSmith::new(String::new(), String::new());
        let data = p.hello();

        assert_eq!("hello", data);
    }
    #[test]
    fn env_vars_loaded() {
        setup();
        let var = std::env::var("POCKETSMITH_USER_ID")
            .expect("Environment variable POCKETSMITH_USER_ID isn't set");

        assert_eq!(var, "161369");
    }
}
