#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub rust_log: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url: String = std::env::var("MONGO_URI").expect("Data url must be set");
        let rust_log: String = std::env::var("RUST_LOG").expect("error in secret");

        Config {
            database_url,
            rust_log,
        }
    }
}
