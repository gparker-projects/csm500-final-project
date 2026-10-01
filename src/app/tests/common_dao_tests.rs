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
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

use sqlx::postgres::{PgPoolOptions}; 
use maple_emr::dao::common_dao::CommonDAO;

use maple_emr::constants;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

/// ### test_get_locations_for_user()
/// 
/// Tests the ability for the DAO to retrieve the locations for a user
/// 
///   Specifically tests: CommonDAO::get_locations_for_user() 
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
    
    // Test 1: Valid Id
    let results: Option< Vec<(i64, String)> > = {CommonDAO::new( db_pool.clone() )}.await.get_locations_for_user( user_id ).await.unwrap();
    match results {
        Some( items ) => {
            let message =  "Expected 35 locations, retrieved ".to_owned() + &items.len().to_string();
            println!( "{}", message );
            assert!( items.len() == 35, "Not enough locations returned");
        },
        None => assert!( false, "No locations returned; expected 35" ),
    };

    // Test 2: Invalid Id
    let results: Option< Vec<(i64, String)> > = {CommonDAO::new( db_pool.clone() )}.await.get_locations_for_user( constants::INVALID_OTHER_ID ).await.unwrap();
    match results {
        Some( items ) => {
            if items.len() > 0 { 
                assert!( false, "{} Locations were returned when none were expected", items.len() );
            }
        },
        None => assert!( true, "No locations returned, as expected" ),
    };
}


/// ### test_get_common_reference()
/// 
/// Tests the ability for the DAO to retrieve 
/// 
///   Specifically tests: CommonDAO::get_common_reference() 
///
#[tokio::test]
async fn test_get_common_reference() {
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

    let mut ref_type_id = 1;

    // Test 1: Valid Id
    let results  = {CommonDAO::new( db_pool.clone() )}.await.get_common_reference( ref_type_id ).await.unwrap();
    match results {
        Some( obj ) => { // SELECT id, name, description
            assert!( obj.0 == 1, "Incorrect Id returned {}", obj.0);
            assert!( obj.1 == "Wound size (length)", "Incorrect name returned {}", obj.1);
            assert!( obj.2 == "Wound size (length)", "Incorrect description returned {}", obj.2); 
        },
        None => assert!( false, "Common Reference (id={}) not returned as expected", ref_type_id),
    };

    // Test 2: Invalid Id
    ref_type_id = constants::INVALID_OTHER_ID;
    let results  = {CommonDAO::new( db_pool.clone() )}.await.get_common_reference( ref_type_id ).await.unwrap();
    match results {
        Some( obj ) => {
            // this function creates a fake entry
            assert!( obj.0 == ref_type_id, "Common Reference (id={}) incorrectly returned id={}", ref_type_id, obj.0);
            assert!( obj.1 == constants::GENERAL_ERROR_NOT_FOUND.to_string(), "Common Reference (id={}) incorrectly returned name={}", ref_type_id, obj.1);
            assert!( obj.2 == constants::GENERAL_ERROR_NOT_FOUND.to_string(), "Common Reference (id={}) incorrectly returned description={}", ref_type_id, obj.2);
        },
        None => assert!( true, "Common Reference (id={}) not returned, as expected", ref_type_id),
    };
}


/// ### test_get_common_references()
/// 
/// Tests the ability for the DAO to retrieve 
///  * Test 1: single group 1, active only
///  * Test 2: single group 1, not only active entries
/// 
///   Specifically tests: CommonDAO::get_common_reference() 
///
#[tokio::test]
async fn test_get_common_references() {
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

    let group_id = 1; 

    // Test 1: single group 1, active only
    match {CommonDAO::new( db_pool.clone() )}.await.get_common_references_by_id( group_id, true ).await.unwrap() {
        Some( items ) => {
            assert!( items.len() == 18, "Expected 18 rows, retrieved {} rows", items.len());
            assert!( items[0].0 == 100043, "Allergies (id=100043), retrieved:'{}'", items[0].0); // this syntax would panic in prod, but for a unit test, it is acceptable
            assert!( items[0].1 == "Allergies", "name='Allergies' expected, retrieved:'{}'", items[0].1);
            assert!( items[0].2 == "Allergies", "description='Allergies' expected, retrieved:'{}'", items[0].2);
        },
        None => assert!( false, "No common references returned; expected 18" ),
    };

    // Test 2: single group 1, not only active entries
    match {CommonDAO::new( db_pool.clone() )}.await.get_common_references_by_id( group_id, false ).await.unwrap() {
        Some( items ) => {
            assert!( items.len() == 18, "Expected 18 rows, retrieved {} rows", items.len());
            assert!( items[0].0 == 100043, "Allergies (id=100043), retrieved:'{}'", items[0].0); // this syntax would panic in prod, but for a unit test, it is acceptable
            assert!( items[0].1 == "Allergies", "name='Allergies' expected, retrieved:'{}'", items[0].1);
            assert!( items[0].2 == "Allergies", "description='Allergies' expected, retrieved:'{}'", items[0].2);
        },
        None => assert!( false, "No common references returned; expected 18" ),
    };

    // Test 3: invalid grouping
    match {CommonDAO::new( db_pool.clone() )}.await.get_common_references_by_id( constants::INVALID_OTHER_ID, false ).await.unwrap() {
        Some( items ) => {
            if items.len() > 0 {
                assert!( false, "No common references expected returned; retrieved {} rows", items.len())
            }
            else{ 
                assert!( true, "Retrieved 0 rows, as expected")
            }
        },
        None => assert!( true, "Retrieved 0 rows, as expected"),
    };
}


/// ### test_get_intervention_statuses()
/// 
/// Tests the ability for the DAO to retrieve 
///     -- just calls get_common_references with a specific type 
/// 
///   Specifically tests: CommonDAO::get_intervention_statuses() 
///
#[tokio::test]
async fn test_get_intervention_statuses() {
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

    match {CommonDAO::new( db_pool.clone() )}.await.get_intervention_statuses().await.unwrap() {
        Some( items ) => {
            assert!( items.len() == 7, "Expected 7 rows, retrieved {} rows", items.len());
            assert!( items[0].0 == 18, "Archived (id=18), retrieved id='{}'", items[0].0); // this syntax would panic in prod, but for a unit test, it is acceptable
            assert!( items[0].1 == "Archived", "name='Archived' expected, retrieved:'{}'", items[0].1);
            assert!( items[0].2 == "Archived", "description='Archived' expected, retrieved:'{}'", items[0].2);
        },
        None => assert!( false, "No intervention statuses returned; expected 18" ),
    };
}


/// ### test_get_clinical_intervention_types()
/// 
/// Tests the ability for the DAO to retrieve 
///     -- just calls get_common_references with a specific type 
/// 
///   Specifically tests: CommonDAO::get_clinical_intervention_types() 
///
#[tokio::test]
async fn test_get_clinical_intervention_types() {
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

    match {CommonDAO::new( db_pool.clone() )}.await.get_clinical_intervention_types().await.unwrap() {
        Some( items ) => {
            assert!( items.len() == 18, "Expected 18 rows, retrieved {} rows", items.len());
            assert!( items[0].0 == 100043, "Allergies (id=100043), retrieved id='{}'", items[0].0); // this syntax would panic in prod, but for a unit test, it is acceptable
            assert!( items[0].1 == "Allergies", "name='Allergies' expected, retrieved:'{}'", items[0].1);
            assert!( items[0].2 == "Allergies", "description='Allergies' expected, retrieved:'{}'", items[0].2);
        },
        None => assert!( false, "No clinical intervention types returned; expected 18" ),
    };
}


/// ### test_get_non_clinical_intervention_types()
/// 
/// Tests the ability for the DAO to retrieve 
///     -- just calls get_common_references with a specific type 
/// 
///   Specifically tests: CommonDAO::get_non_clinical_intervention_types() 
///
#[tokio::test]
async fn test_get_non_clinical_intervention_types() {
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

    match {CommonDAO::new( db_pool.clone() )}.await.get_non_clinical_intervention_types().await.unwrap() {
        Some( items ) => {
            assert!( items.len() == 7, "Expected 7 rows, retrieved {} rows", items.len());
            assert!( items[0].0 == 100040, "Alerts/CCI/SPI (id=100040), retrieved id='{}'", items[0].0); // this syntax would panic in prod, but for a unit test, it is acceptable
            assert!( items[0].1 == "Alerts/CCI/SPI", "name='Alerts/CCI/SPI' expected, retrieved:'{}'", items[0].1);
            assert!( items[0].2 == "Alerts/CCI/SPI", "description='Alerts/CCI/SPI' expected, retrieved:'{}'", items[0].2);
        },
        None => assert!( false, "No clinical intervention types returned; expected 7" ),
    };
}

/// ### test_get_intervention_type()
/// 
/// Tests the ability for the DAO to retrieve 
///     -- just calls get_common_reference with a specific type 
/// 
///   Specifically tests: CommonDAO::get_intervention_type() 
///
#[tokio::test]
async fn test_get_intervention_type() {
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

    let type_id = 100046;

    match {CommonDAO::new( db_pool.clone() )}.await.get_intervention_type(type_id).await.unwrap() {
        Some( item ) => {
            assert!( item.0 == 100046, "Documents (id=100046), retrieved id='{}'", item.0); // this syntax would panic in prod, but for a unit test, it is acceptable
            assert!( item.1 == "Documents", "name='Documents' expected, retrieved:'{}'", item.1);
            assert!( item.2 == "Documents", "description='Documents' expected, retrieved:'{}'", item.2);
        },
        None => assert!( false, "No intervention type returned; expected 1 (Documents)" ),
    };
}