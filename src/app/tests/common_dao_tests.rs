/// #Unit & Integration tests for the CommonDAO module
/// 
/// * new()
/// * get_locations_for_user()
/// * get_common_reference()
///   * get_intervention_type() <- calls get_common_reference with a specific type
///
/// * get_common_references()
///   * get_intervention_statuses()            <- these all call get_common_references with a specific type
///   * get_clinical_intervention_types()      <- 
///   * get_non_clinical_intervention_types()  <-
/// 
/// * is_intervention_group_type()
///
/// #### Refs
/// * Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy

mod common;

use sqlx::postgres::{PgPoolOptions}; 
use tracing;
use maple_emr::dao::common_dao::CommonDAO;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

/// ### get_locations_for_user()
/// 
/// Tests the ability for the DAO to retrieve the locations for a user
/// 
///   Specifically tests: CommonDAO::get_current_encounter() 
///
#[tokio::test]
async fn test_get_locations_for_user() {
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::debug!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };

    let user_id = 1;

    // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
    // does not instantiate however, the remainder of this test will fail.
    let cdao = CommonDAO::new( db_pool );
    
    let results: Option< Vec<(i64, String)> > = cdao.await.get_locations_for_user( user_id ).await.unwrap();
    match results {
        Some( items ) => {
            let message =  "Expected 35 locations, retrieved ".to_owned() + &items.len().to_string();
            println!( "{}", message );
            assert!( items.len() == 35, "Not enough locations returned");
        },
        None => assert!( false, "No locations returned; expected 35" ),
    };
}