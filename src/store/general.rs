use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        match PgPoolOptions::new()
            .max_connections(8)
            .connect(db_url)
            .await
        {
            Ok(pool) => return Store { connection: pool },
            Err(_e) => panic!("Couldn't establish DB connection! Error: {}", _e),
        };
    }
}