use sqlx::postgres::{PgPoolOptions, PgPool, PgRow};
use sqlx::Row;
//use serde::{Deserialize, Serialize};

//use crate::dto::user;
use crate::dto::user::dto::User;

//  B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85

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

    /// Checks the user is in the database, and that the password matches (TODO)
    /// Returns a true/false value
    /// 
    pub async fn can_user_login(&self,
                            user_name: String, 
                            user_password: String,
                          ) -> Result< Option<User>, std::io::Error> {

        // query the database for a user that matches the username and password
        // columns MUST be lowercase and mapped as such below, Rust can not translate them
        let query = format!("SELECT id, name, username, email, created_at, password FROM USERS WHERE USERNAME = '{}' AND PASSWORD = '{}'", user_name, user_password);

        //println!("Query: {}", query);

        match sqlx::query(&query)
        .fetch_optional(&self.connection)
        .await
        {
            Ok( Some(row) ) => {
                println!("Successful login (results found) for: {}", user_name);
                Ok( Some (   {
                        let new_id: i64 = row.get("id");  // Rust to Postgresql mappings: https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html
                        let created_at: chrono::NaiveDateTime = row.get("created_at");

                        User {
                            id: new_id,
                            name: row.get("name"),
                            user_name: row.get("username"),
                            email: row.get("email"),
                            created_timestamp: created_at, 
                            password: row.get("password"),
                        }
                    }
                ) )
            }
            Ok(None) => {
                println!("No user found for: {}", user_name);
                Ok( None )
            }
            Err(err) => {
                println!("Error on login for: {} ({})", user_name, err);
                Ok( None )
            }
        }
    }   


    ///
    /// Given a user id, obtain all the user permissions that user has. Contains the linkages between the department, permission and user.
    /// 
    pub async fn get_user_permissions(&self, user_id: i64 ) -> Result< Option<UserAuthorization>, std::io::Error> {
        // construct query - we have a denormalized data structure here to save joins, so the table has all the Id's someone would ever need
        // first user is users_id = 3
        let query = format!("SELECT department_id, permission_id FROM public.user_permission where active_flag = 'Y' and users_id = {} group by department_id, permission_id order by permission_id", user_id);
        //println!("Query: {}", query);

        match sqlx::query(&query)
        .fetch_optional(&self.connection)
        .await
        {
            Ok( Some(row) ) => {
                println!("Successful Authorization (results found) for: {}", user_id);
                Ok( Some (   {
                        let new_id: i64 = row.get("id");  // Rust to Postgresql mappings: https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html
                        let created_at: chrono::NaiveDateTime = row.get("created_at");

                        User {
                            id: new_id,
                            name: row.get("name"),
                            user_name: row.get("username"),
                            email: row.get("email"),
                            created_timestamp: created_at, 
                            password: row.get("password"),
                        }
                    }
                ) )
            }
            Ok(None) => {
                println!("No Authorization found for: {}", user_id);
                Ok( None )
            }
            Err(err) => {
                println!("Error on collect Authorization for: {} ({})", user_id, err);
                Ok( None )
            }
        }
    }    
    
}