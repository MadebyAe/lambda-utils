use aws_config::SdkConfig;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::update_item::UpdateItemError;
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

/// `condition_expression` is optional. A failed condition comes back as
/// `Err(SdkError::ServiceError)` with `.err().is_conditional_check_failed_exception()`.
pub async fn update_item(
    key: HashMap<String, AttributeValue>,
    update_expression: &str,
    condition_expression: Option<&str>,
    expression_attribute_names: Option<HashMap<String, String>>,
    expression_attribute_values: Option<HashMap<String, AttributeValue>>,
) -> Result<(), Box<SdkError<UpdateItemError>>> {
    let config: SdkConfig = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let table_name = get_dynamodb_table_from_env_var().unwrap();

    client
        .update_item()
        .table_name(table_name)
        .set_key(Some(key))
        .update_expression(update_expression)
        .set_condition_expression(condition_expression.map(str::to_string))
        .set_expression_attribute_names(expression_attribute_names)
        .set_expression_attribute_values(expression_attribute_values)
        .send()
        .await
        .map_err(Box::new)?;

    Ok(())
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

    #[tokio::test]
    #[ignore = "requires a real (or local-emulator) DynamoDB endpoint"]
    async fn test_update_item_applies_a_conditional_transition() {
        env::set_var(get_dynamodb_table_env_key(), "test-table");

        let key = || {
            HashMap::from([(
                "id".to_string(),
                AttributeValue::S("test-conditional".to_string()),
            )])
        };

        let mut item = key();
        item.insert(
            "status".to_string(),
            AttributeValue::S("PENDING".to_string()),
        );
        put_item(item).await.unwrap();

        let names = || HashMap::from([("#status".to_string(), "status".to_string())]);
        let values = || {
            HashMap::from([
                (
                    ":new".to_string(),
                    AttributeValue::S("IN_PROGRESS".to_string()),
                ),
                (
                    ":expected".to_string(),
                    AttributeValue::S("PENDING".to_string()),
                ),
            ])
        };

        // First call: item is still PENDING, condition holds, update applies.
        update_item(
            key(),
            "SET #status = :new",
            Some("#status = :expected"),
            Some(names()),
            Some(values()),
        )
        .await
        .unwrap();

        let after_first = get_item(key()).await.unwrap().unwrap();
        assert_eq!(
            after_first.get("status"),
            Some(&AttributeValue::S("IN_PROGRESS".to_string()))
        );

        // Second call with the same condition: item is now IN_PROGRESS, so
        // the condition (#status = PENDING) no longer holds — this is the
        // exact scenario a duplicate SQS delivery hits. The caller (not this
        // crate) is responsible for recognizing this as "already handled."
        let second = update_item(
            key(),
            "SET #status = :new",
            Some("#status = :expected"),
            Some(names()),
            Some(values()),
        )
        .await;

        let err = second.expect_err("condition should fail — item was already IN_PROGRESS");
        match *err {
            SdkError::ServiceError(ref e) => {
                assert!(e.err().is_conditional_check_failed_exception());
            }
            other => panic!("expected a ConditionalCheckFailedException, got {other:?}"),
        }
    }
}
