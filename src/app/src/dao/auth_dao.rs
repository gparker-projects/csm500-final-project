//!
//!  CSM500 Project (April - October 2026)
//!  Graham Parker (Student ID: 240120522)

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

    /// ### AuthDAO::new()
    ///   Creates a new AuthObjects object, with a database pool for use by other calls
    /// 
    /// #### Parameters:
    /// * db_connection (PgPool): a PgPool for establishing a database connection
    /// 
    /// #### Returns:
    /// * AuthDAO: the AuthDAO object that was created
    /// 
    pub async fn new(db_connection: PgPool) -> Self {
        AuthDAO {
            connection: db_connection,
        }
    }


    /// ### can_user_login()
    ///   Checks the user is in the database, and that the password matches
    /// 
    /// #### Parameters:
    /// * user_name (i64): the name of the user which is being authenticated
    /// * user_password (i64): the password of the user which is being authenticated
    /// 
    /// #### Returns:
    /// * Option<User>: the User, if found
    /// * sqlx::Error: An error, if applicable
    /// 
    pub async fn can_user_login(&self,
                            user_name: String, 
                            user_password: String,
                        ) -> Result< Option<User>, std::io::Error> {

        // query the database for a user that matches the username and password
        // columns MUST be lowercase and mapped as such below, Rust can not translate them
        let query_level_0 = db_query::QRY_USER_LOGIN.replace("{user_name}", &user_name);
        let query = query_level_0.replace("{user_password}", &user_password);

        tracing::debug!("Query: {}", query);
        //println!("Query: {}", query);

        match sqlx::query(&query)
        .fetch_optional(&self.connection)
        .await
        {
            Ok( Some(row) ) => {
                tracing::debug!("Successful login (results found) for: {}", user_name);

                // we would perform a permissions check here, but as it is login on, we already have access to the user_id and their
                // permissions directly in the database. Let's use that (above) instead of trying to hit the internal UserAuthentication
                // structure, which has not actually been assembled at this point.

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
            },
            _ => {
                tracing::debug!("No user found for: {}", user_name);
                Ok( None )
            },
        }
    }   


    /// ### get_user_permissions()
    ///   Given a user id, obtain all the user permissions that user has. Contains the linkages between the department, permission and user.
    /// 
    /// #### Parameters:
    /// * user_id (i64): the id of the user for which permissions are being retrieved
    /// 
    /// #### Returns:
    /// * Option<UserAuthorization>: a vector of UserAuthorizations, if found
    /// * std::io::Error: An error, if applicable
    /// 
    pub async fn get_user_permissions(&self, user_id: i64 ) -> Result< Option<UserAuthorization>, std::io::Error> {
        // construct query - we have a denormalized data structure here to save joins, so the table has all the Id's someone would ever need
        let query = db_query::QRY_USER_PERMISSIONS_ALL_ACTIVE.replace("{user_id}", &user_id.to_string());
        tracing::debug!("get_user_permissions Query: {}", query);
        //println!("get_user_permissions Query: {}", query);

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

    /// ### get_user_and_departments_at_current_user_sites()
    ///   Accessor to retrive User and Description entries from the database into a tuple.
    /// 
    /// #### Parameters:
    /// * user_id (i64): the id of the user, which will be used to locate the department and related users for the list
    /// 
    /// #### Returns:
    /// * Option< Vec<(i64, String, String)> >: a vector of tuples containing:
    ///        - user_id (i64): id number of the user in the list
    ///        - name (String): name of the user in the list
    ///        - department_name (String): name of the department in the list 
    ///
    pub async fn get_user_and_departments_at_current_user_sites(&self, user_id: i64)-> Result< Option< Vec<(i64, String, String)> >, std::io::Error> {
        tracing::debug!("get_user_and_departments_at_current_user_sites()");
        let query = db_query::QRY_ALL_USERS_AND_DEPARTMENT_NAME.replace("{user_id}", &user_id.to_string());

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