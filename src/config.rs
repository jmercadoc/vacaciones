use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub dynamodb_table_name: String,
    pub session_secret: String,
    pub session_ttl_days: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenv::dotenv().ok();

        Ok(Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .unwrap_or(3000),
            dynamodb_table_name: env::var("DYNAMODB_TABLE_NAME")
                .unwrap_or_else(|_| "vacaciones".into()),
            session_secret: env::var("SESSION_SECRET")
                .unwrap_or_else(|_| "change-me-in-production".into()),
            session_ttl_days: env::var("SESSION_TTL_DAYS")
                .unwrap_or_else(|_| "7".into())
                .parse()
                .unwrap_or(7),
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
