use aws_config::SdkConfig;
use aws_sdk_sqs::operation::receive_message::ReceiveMessageOutput;
use aws_sdk_sqs::{Client, Error as SqsError};
use std::env;
use std::error::Error;

fn get_sqs_url_from_env_var() -> Result<String, Box<dyn Error + Send + Sync>> {
    return env::var(get_sqs_url_env_key())
        .map_err(|_| "Missing AWS_SQS_URL environment var".into());
}

fn get_sqs_url_env_key() -> &'static str {
    return "AWS_SQS_URL";
}

pub async fn delete_message_from_sqs(receipt_handle: &str) -> Result<(), SqsError> {
    let config: SdkConfig = aws_config::load_from_env().await;
    let sqs_client = Client::new(&config);
    let queue_url = get_sqs_url_from_env_var().unwrap();

    sqs_client
        .delete_message()
        .queue_url(queue_url)
        .receipt_handle(receipt_handle)
        .send()
        .await?;

    Ok(())
}

pub async fn send_to_sqs(data: String) -> Result<(), SqsError> {
    let config: SdkConfig = aws_config::load_from_env().await;
    let sqs_client = Client::new(&config);
    let message_body = &data;
    let queue_url = get_sqs_url_from_env_var().unwrap();

    sqs_client
        .send_message()
        .queue_url(&queue_url)
        .message_body(message_body)
        .send()
        .await?;

    Ok(())
}

pub async fn receive_from_sqs() -> Result<ReceiveMessageOutput, SqsError> {
    let config: SdkConfig = aws_config::load_from_env().await;
    let sqs_client = Client::new(&config);
    let queue_url = get_sqs_url_from_env_var().unwrap();
    let data = sqs_client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    fn create_mock_object() -> Value {
        json!({ "hello": "world" })
    }

    #[tokio::test]
    #[ignore = "ignore for now"]
    async fn test_send_to_sqs() {
        env::set_var(
            get_sqs_url_env_key(),
            "https://sqs.us-east-1.amazonaws.com/0000000000/foo-bar",
        );

        let data = create_mock_object();
        let client = send_to_sqs(data.to_string()).await.unwrap();

        assert_eq!(client, ());
    }

    #[tokio::test]
    #[ignore = "ignore for now"]
    async fn test_receive_from_sqs() {
        env::set_var(
            get_sqs_url_env_key(),
            "https://sqs.us-east-1.amazonaws.com/0000000000/foo-bar",
        );

        let data: ReceiveMessageOutput = receive_from_sqs().await.unwrap();

        assert_eq!(data.messages.unwrap().len(), 0);
    }
}
