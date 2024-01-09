use mongodb::{Client};
use mongodb::options::{ClientOptions, ResolverConfig};
use once_cell::sync::OnceCell;
use std::env;
use std::error::Error;

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();

pub async fn create_mongodb_client() {
    let mongodb_uri = get_mongodb_url_from_env_var().unwrap();
    let client_options = ClientOptions::parse_with_resolver_config(mongodb_uri, ResolverConfig::cloudflare()).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    return MONGODB_CLIENT.set(client).expect("Should set MongoDB client");
}

fn get_mongodb_url_from_env_var() -> Result<String, Box<dyn Error + Send + Sync>> {
    let mongodb_uri = match env::var(get_mongodb_uri_env_key()) {
        Err(_) => return Err("Missing MONGODB_URI environment var".into()),
        Ok(result) => Ok(result),
    };

    return mongodb_uri;
}

pub fn get_mongodb_uri_env_key() -> &'static str {
    return "MONGODB_URI";
}

pub fn get_mongodb_client() -> Result<&'static Client, Box<dyn Error + Send + Sync>> {
    return MONGODB_CLIENT.get().ok_or_else(|| "Missing MongoDB client as static reference".into());
}

/*
#[cfg(test)]
use super::*;
use mongodb::*;
use mongodb::bson::{doc, Document};
use serde_json::{to_string};

#[tokio::test]
async fn test_create_mongodb_client() {
    create_mongodb_client().await;

    let ip = "127.0.0.1";
    let client = get_mongodb_client().unwrap();
    let database = client.database("ae");
    let collection: Collection<Document> = database.collection("logs");
    let filter = doc! { "ip": ip };
    let cursor = collection.find_one(filter, None).await.unwrap();
    let output = to_string(&cursor).unwrap();

    assert!(output.contains("ip"));
    assert!(output.contains(ip));
}
*/
