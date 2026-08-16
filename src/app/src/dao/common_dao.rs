use crate::dao::db_query;

use sqlx::postgres::{PgPoolOptions, PgPool}; 
//use sqlx::Row;
//use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct CommonDAO {
    pub connection: PgPool,
}

impl CommonDAO {
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

        CommonDAO {
            connection: db_pool,
        }
    }

    ///
    /// Accessor to retrive locations from the database into a tuple. 
    ///
    /// Returns: a tuple (i64, String) containing the id of the location and an aggregated string
    ///          describing the location.
    /// 
    pub async fn get_locations(&self)-> Result< Option< Vec<(i64, String)> >, std::io::Error> {
        let query: String = db_query::QRY_ACTIVE_LOCATIONS.to_owned();

        println!("get_locations()");

        let rows: Vec<( i64, String )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();
        if rows.is_empty() {
            println!("No Locations defined in system");
            return Ok( Some( Vec::new() ) );
        }
        else{
            let mut results: Vec<(i64, String)> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_loc_id: i64 = row.0; // location_id
                let tmp_aggregate_name = row.1; // aggregated name

                results.push( (tmp_loc_id, tmp_aggregate_name) );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    ///
    /// Accessor to retrive locations from the database into a tuple. 
    ///
    /// Returns: a tuple (i64, String) containing the id of the location and an aggregated string
    ///          describing the location.
    /// 
    pub async fn get_locations_for_user(&self, user_id: i64)-> Result< Option< Vec<(i64, String)> >, std::io::Error> {
        let query_level_0: String = db_query::QRY_CURRENT_USER_LOCATIONS.to_owned();
        let query = query_level_0.replace("{}", &user_id.to_string());

        println!("get_locations_for_user()");

        let rows: Vec<( i64, String )> = sqlx::query_as(&query)
                                                .fetch_all(&self.connection) 
                                                .await
                                                .unwrap_or_default();
        if rows.is_empty() {
            println!("No Locations defined in system");
            return Ok( Some( Vec::new() ) );
        }
        else{
            let mut results: Vec<(i64, String)> = Vec::with_capacity(rows.len());
            for row in rows {
                let tmp_loc_id: i64 = row.0; // location_id
                let tmp_aggregate_name = row.1; // aggregated name

                results.push( (tmp_loc_id, tmp_aggregate_name) );
            }
            return Ok( Some( results ) ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }
}