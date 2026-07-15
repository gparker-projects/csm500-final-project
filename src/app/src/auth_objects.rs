use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::Row;
use serde::{Deserialize, Serialize};
//mod errors;

//  B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct AuthObjects {
    pub connection: PgPool,
}

impl AuthObjects {
    pub async fn new(db_url: &str) -> Self {

        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("{}", e),
        };

        AuthObjects {
            connection: db_pool,
        }
    }

    pub async fn get_users(&self,
                           limit: Option<i32>,
                           offset: i32, 
                          ) -> Result< Vec<User>, std::io::Error> {

        match sqlx::query("SELECT * from USERS LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                //created_at: row.get("created_at"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(results_users) => Ok(results_users),
             Err(e) => panic!("{}", e),
        }
    }
}
