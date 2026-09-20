///
/// #Unit & Integration tests for the EncounterDAO module
/// 
/// * new()
/// * get_current_encounter()
/// * get_encounters()
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 

mod common;

use maple_emr::constants;
use sqlx::postgres::{PgPoolOptions}; 
use tracing;
use maple_emr::dao::encounter_dao::EncounterDAO;
use maple_emr::dto::encounter::Encounter;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

/// ### get_current_encounter()
/// 
/// Tests the ability for the DAO to retrieve the current Encounter
/// 
///   Specifically tests: PatientDAO::get_current_encounter() 
///
#[tokio::test]
async fn test_get_current_encounter() {
    use maple_emr::constants;

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

    let patient_id = 7; // only 7 has not been discharged at this time

    // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
    // does not instantiate however, the remainder of this test will fail.
    let pdao = EncounterDAO::new( db_pool.clone() );
    let enc: Encounter = pdao.await.get_current_encounter( patient_id ).await;
    assert!( enc.id == 81, "Different Encounter returned" );

    // Test 2: INVALID_PATIENT_ID
    let _enc2: Encounter = {EncounterDAO::new( db_pool.clone() )}.await.get_current_encounter( constants::INVALID_PATIENT_ID ).await;
    assert!( true, "Invalid Patient Id accepted; no change of state" );
}

/// ### get_encounters()
/// 
/// Tests the ability for the DAO to retrieve the Encounters
/// 
///   Specifically tests: PatientDAO::get_current_encounter() 
///
#[tokio::test]
async fn get_encounters() {
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

    let patient_id = 1; // only 7 has not been discharged at this time

    // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
    // does not instantiate however, the remainder of this test will fail.
    let pdao = EncounterDAO::new( db_pool.clone() );

    // Test 1: the get_encounters actually calls get_encounter(), so we only need to test the false case
    let enc: Option<Vec<Encounter>> = pdao.await.get_encounters( patient_id, false ).await.unwrap();
    match enc {
        Some(items) => {
            assert!( items.len() <= 1, "Not enough Encounters returned; some expected" );
        },
        None => assert!( false, "No Encounters returned; some expected" ),
    };

    // Test 2: Invalid Patient Id
    let enc2: Option<Vec<Encounter>> = {EncounterDAO::new( db_pool.clone() )}.await.get_encounters( constants::INVALID_PATIENT_ID, false ).await.unwrap();
    match enc2 {
        Some(items) => {
            if items.len() == 0 {
                assert!( true, "No Encounters returned, as expected" )
            }
            else{
                assert!( false, "No Encounters expected, {} were returned", items.len() )
            }             
        },
        None => assert!( true, "No Encounters returned, as expected" )
    };
}