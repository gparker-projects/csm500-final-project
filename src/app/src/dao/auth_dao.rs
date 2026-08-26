use sqlx::postgres::{PgPool}; //, PgRow};
use sqlx::Row;
use std::io::{Error, ErrorKind};
use tracing;

use crate::dao::db_query;
use crate::dto::user::User;
use crate::dto::user_auth::*;

//  B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85

#[derive(Debug, Clone)]
pub struct AuthDAO {
    pub connection: PgPool,
}

impl AuthDAO {
    /// Creates a new AuthObjects object, with a database pool for use by other calls
    /// 
    pub async fn new(db_connection: PgPool) -> Self {
        AuthDAO {
            connection: db_connection,
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
        let query = format!("SELECT id, name, username, email, created_timestamp, password FROM USERS WHERE USERNAME = '{}' AND PASSWORD = '{}'", user_name, user_password);

        tracing::debug!("Query: {}", query);

        match sqlx::query(&query)
        .fetch_optional(&self.connection)
        .await
        {
            Ok( Some(row) ) => {
                tracing::debug!("Successful login (results found) for: {}", user_name);
                Ok( Some (   {
                        let tmp_new_id: i64 = row.get("id");  // Rust to Postgresql mappings: https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html
                        let tmp_created_at: chrono::NaiveDateTime = row.get("created_timestamp");

                        User {
                            id: tmp_new_id,
                            name: row.get("name"),
                            user_name: row.get("username"),
                            email: row.get("email"),
                            created_timestamp: tmp_created_at, 
                            password: row.get("password"),
                        }
                    }
                ) )
            }
            Ok(None) => {
                tracing::debug!("No user found for: {}", user_name);
                Ok( None )
            }
            Err(err) => {
                tracing::error!("Error on login for: {} ({})", user_name, err);
                Ok( None )
            }
        }
    }   

    ///
    /// Given a user id, obtain all the user permissions that user has. Contains the linkages between the department, permission and user.
    /// 
    pub async fn get_user_permissions(&self, user_id: i64 ) -> Result< Option<UserAuthorization>, std::io::Error> {
        // construct query - we have a denormalized data structure here to save joins, so the table has all the Id's someone would ever need
        let query = format!("SELECT department_id, permission_id FROM public.user_permission where active_flag = 'Y' and users_id = {} group by department_id, permission_id order by permission_id", user_id);
        tracing::debug!("get_user_permissions Query: {}", query);

        // https://docs.rs/sqlx/latest/sqlx/fn.query_as.html
        // https://stackoverflow.com/questions/67243108/mapping-nm-relations-into-vec-using-sqlx
        //
        let rows: Vec<(i64, i64)> = sqlx::query_as(&query)
        .fetch_all(&self.connection)
        .await
        .unwrap_or_default(); 

        if rows.is_empty() {
            let errmsg = format!("No permissions found for user_id: {}", user_id);
            tracing::error!("{}", errmsg); // had to use https://doc.rust-lang.org/std/io/struct.Error.html to return Error here
            return Err(Error::new(ErrorKind::Other, errmsg));
        }

        let mut perms: Vec<Permission> = Vec::with_capacity(rows.len());
        for row in rows {
            let tmp_dept_id: i64 = row.0;
            let tmp_perm_id: i64 = row.1;

            perms.push(
                Permission {
                    department_id: tmp_dept_id,
                    permission_id: tmp_perm_id,
                }
            );
        }
        
        let result = UserAuthorization {
            granted_permissions: perms
        };
        Ok(Some(result))
        
    }  

    ///
    /// Accessor to retrive User and Description entries from the database into a tuple.
    ///
    /// Returns: a tuple (i64, String) containing the id of the location and an aggregated string
    ///          describing the location.
    ///
    pub async fn get_user_and_departments_at_current_user_sites(&self, user_id: i64)-> Result< Option< Vec<(i64, String, String)> >, std::io::Error> {
        let query_level_0: String = db_query::QRY_ALL_USERS_AND_DEPARTMENT_NAME.to_owned();
        let query = query_level_0.replace("{user_id}", &user_id.to_string());

        tracing::debug!("get_user_and_departments_at_current_user_sites()");

        let rows: Vec<( i64, String, String )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();
        if rows.is_empty() {
            tracing::error!("Users and departments not found for user_id={}", user_id);
            return Ok( Some( Vec::new() ) );
        }
        else{
            let mut results: Vec<(i64, String, String)> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_id: i64 = row.0; // user_id
                results.push( (tmp_id, row.1, row.2) ); // user_id, name, department_name
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }
    
}