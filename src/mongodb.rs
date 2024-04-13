use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::Client;
use once_cell::sync::OnceCell;
use std::env;
use std::error::Error;

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();

pub async fn create_mongodb_client() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mongodb_uri = get_mongodb_url_from_env_var().unwrap();
    let client_options =
        ClientOptions::parse_with_resolver_config(mongodb_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    let client = Client::with_options(client_options).unwrap();

    return MONGODB_CLIENT
        .set(client)
        .map_err(|_| "MongoDB client already set".into());
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
    return MONGODB_CLIENT
        .get()
        .ok_or_else(|| "Missing MongoDB client as static reference".into());
}

#[cfg(test)]
mod mongodb_tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_create_mongodb_client() {
        env::set_var(
            get_mongodb_uri_env_key(),
            "mongodb+srv://foo:bar@cluster0.irqdk.mongodb.net/test",
        );

        let client = create_mongodb_client().await;

        assert!(client.is_ok());
        assert!(MONGODB_CLIENT.get().is_some());
    }
}
