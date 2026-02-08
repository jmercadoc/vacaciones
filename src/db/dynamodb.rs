use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use crate::config::Config;

#[derive(Clone, Debug)]
pub struct DynamoDBClient {
    pub client: Client,
    pub table_name: String,
}

impl DynamoDBClient {
    pub async fn new(config: &Config) -> Self {
        let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;

        DynamoDBClient {
            client: Client::new(&aws_config),
            table_name: config.dynamodb_table_name.clone(),
        }
    }
}