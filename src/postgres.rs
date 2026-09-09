use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::error::Error;

/// Re-exported for convenience so consumers of the `pgvector` feature don't
/// need to add the `pgvector` crate as a direct dependency themselves —
/// implements `sqlx::Type`/`Encode`/`Decode` for Postgres's `vector` column
/// type, usable directly in `sqlx::query!`/`query_as!` against a pool from
/// `get_postgres_client()`.
#[cfg(feature = "pgvector")]
pub use pgvector::Vector;

static POSTGRES_CLIENT: OnceCell<PgPool> = OnceCell::new();

pub async fn create_postgres_client() -> Result<(), Box<dyn Error + Send + Sync>> {
    let postgres_url = get_postgres_url_from_env_var()?;
    let pool = PgPoolOptions::new().connect(&postgres_url).await?;

    POSTGRES_CLIENT
        .set(pool)
        .map_err(|_| "Postgres client already set".into())
}

fn get_postgres_url_from_env_var() -> Result<String, Box<dyn Error + Send + Sync>> {
    env::var(get_postgres_url_env_key()).map_err(|_| "Missing POSTGRES_URL environment var".into())
}

pub fn get_postgres_url_env_key() -> &'static str {
    "POSTGRES_URL"
}

pub fn get_postgres_client() -> Result<&'static PgPool, Box<dyn Error + Send + Sync>> {
    POSTGRES_CLIENT
        .get()
        .ok_or_else(|| "Missing Postgres client as static reference".into())
}

#[cfg(test)]
mod postgres_tests {
    use super::*;
    use std::env;

    #[tokio::test]
    #[ignore = "requires a real (or local-emulator) Postgres connection"]
    async fn test_create_postgres_client() {
        env::set_var(
            get_postgres_url_env_key(),
            "postgres://postgres:postgres@localhost/postgres",
        );

        let client = create_postgres_client().await;

        assert!(client.is_ok());
        assert!(POSTGRES_CLIENT.get().is_some());
    }
}
