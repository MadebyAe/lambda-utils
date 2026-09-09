use aws_config::SdkConfig;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn get_dynamodb_table_from_env_var() -> Result<String, Box<dyn Error + Send + Sync>> {
    env::var(get_dynamodb_table_env_key())
        .map_err(|_| "Missing AWS_DYNAMODB_TABLE environment var".into())
}

pub fn get_dynamodb_table_env_key() -> &'static str {
    "AWS_DYNAMODB_TABLE"
}

pub async fn put_item(
    item: HashMap<String, AttributeValue>,
) -> Result<(), Box<SdkError<PutItemError>>> {
    let config: SdkConfig = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let table_name = get_dynamodb_table_from_env_var().unwrap();

    client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await
        .map_err(Box::new)?;

    Ok(())
}

pub async fn get_item(
    key: HashMap<String, AttributeValue>,
) -> Result<Option<HashMap<String, AttributeValue>>, Box<SdkError<GetItemError>>> {
    let config: SdkConfig = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let table_name = get_dynamodb_table_from_env_var().unwrap();

    let output = client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
        .map_err(Box::new)?;

    Ok(output.item)
}

pub async fn delete_item(
    key: HashMap<String, AttributeValue>,
) -> Result<(), Box<SdkError<DeleteItemError>>> {
    let config: SdkConfig = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let table_name = get_dynamodb_table_from_env_var().unwrap();

    client
        .delete_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
        .map_err(Box::new)?;

    Ok(())
}

#[cfg(test)]
mod dynamodb_tests {
    use super::*;
    use std::env;

    #[tokio::test]
    #[ignore = "requires a real (or local-emulator) DynamoDB endpoint"]
    async fn test_put_and_get_item() {
        env::set_var(get_dynamodb_table_env_key(), "test-table");

        let mut item = HashMap::new();
        item.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

        put_item(item.clone()).await.unwrap();

        let mut key = HashMap::new();
        key.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

        let result = get_item(key).await.unwrap();
        assert!(result.is_some());
    }
}
