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

    /// Creates a new AuthObjects object, with a database pool for use by other calls
    /// 
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

    /// Returns a list of users from the database
    /// 
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

    /// Checks the user is in the database, and that the password matches (TODO)
    /// Returns a true/false value
    /// 
    pub async fn can_user_login(&self,
                            user_name: String, 
                            user_password: String,
                          ) -> Result< bool, std::io::Error> {

        #[derive(sqlx::FromRow)]
        struct SingleResult{
            pub count: i64,
        }

        let query = format!("SELECT COUNT(ID) FROM USERS WHERE USERNAME = '{}'", user_name);// and PASSWORD = '$2'");

        println!("Query: {}", query);

        match sqlx::query(&query)
          //  .bind(limit)
          //  .bind(offset)a
            .map(|row: PgRow| SingleResult {
                count: row.get("count"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(results) => {
                if results[0].count == 0{
                    println!("No results for: {} ({})", user_name, results[0].count);
                    Ok(false)
                }
                else {
                    println!("Successful login (results found) for: {}", user_name);
                    Ok(true)
                }
            } 
            Err(_e) => {
                println!("Error on login for: {}", user_name);
                Ok(false)
            }
        }
    }
    
}
