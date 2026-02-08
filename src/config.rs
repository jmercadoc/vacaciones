use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub dynamodb_table_name: String,
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
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
