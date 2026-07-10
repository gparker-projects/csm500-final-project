
use serde::{Deserialize, Serialize};
//https://github.com/pykeio/ort/blob/main/examples/sentence-transformers/semantic-similarity.rs

pub struct NLP {
    
}

#[derive(Debug, Clone)]
impl NLP {

    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        AuthObjects {
            connection: db_pool,
        }
    }

    pub async fn execute( &self ) -> Result< Vec<User>, std::io::Error> {

    }

}