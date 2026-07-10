use sqlx::postgres::{PgPoolOptions, PgPool}; //, PgRow};
use serde::{Deserialize, Serialize};

//  B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: String,
}

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_users( &self, limit: Option<i32>, offset: i32, ) -> Result< Vec<User>, std::io::Error> {

        let rows = match sqlx::query("SELECT * from users LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.connection)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };
    }
}
