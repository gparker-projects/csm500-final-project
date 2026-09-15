///
/// Unit & Integration tests for the InterventionDAO module:
/// 
/// * new()
/// * get_intervention
/// * get_interventions
/// * get_all_intervention_details_for_an_intervention
/// * get_most_recent_vitals
/// * upsert_intervention_from_intv_form
/// * upsert_intervention_details_from_intv_form
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 

mod common;

use chrono::{Utc, NaiveDateTime};
use sqlx::postgres::{PgPoolOptions}; 
use tracing;

use maple_emr::constants;
use maple_emr::dao::intervention_dao::InterventionDAO;
use maple_emr::dto::intervention::Intervention;
use maple_emr::dto::intervention_detail::InterventionDetail;
use maple_emr::ui::data_forms::*;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

///
/// Tests the ability for the DAO to CREATE, SELECT and UPDATE Intervention Detail records
///  * InterventionDAO::new()
///  * upsert_intervention_details_from_intv_form()
///  * get_all_intervention_details_for_an_intervention()
/// 
#[tokio::test]
async fn test_ins_get_upd_intervention_details(){
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
          tracing::warn!("{}", e);
          panic!("{}", e)
        },
    };

    let test_user_id = 2;
    let mut tmp_id: i64 = constants::INVALID_OTHER_ID;

    //println!("Testing: upsert_intervention_details_from_intv_form(): first insertion");

    // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
    // does not instantiate however, the remainder of this test will fail.
    let idao = InterventionDAO::new( db_pool.clone() ).await;
    let mut frm: InterventionDetailsDataForm = Default::default();
    frm.intervention_details_id = constants::NOT_SPECIFIED_ID.to_string();
    frm.type_id = "9".to_string();
    frm.intervention_id = "1".to_string(); 
    frm.value = "Not Detected".to_string();
    frm.notes = "Unit test".to_string();

    //Step 1: create a record
    let insert_ivdtls_results = idao.upsert_intervention_details_from_intv_form(frm.clone(), test_user_id).await.unwrap();
    if insert_ivdtls_results == constants::NOT_SPECIFIED_ID{
        assert!(false); // did not receive a new id
    }
    else{ 
        tmp_id = insert_ivdtls_results; // returns the indet
        assert!(true);
    }

    let mut new_intv_dtls: InterventionDetail = Default::default(); // have an 
    let intervention_id: i64 = 1;

    //Step 2: get it back
    let qry_results = idao.get_all_intervention_details_for_an_intervention(intervention_id, constants::NOT_SPECIFIED_ID).await;
    let tmp_ivec = qry_results.unwrap().clone();
    match tmp_ivec.clone(){
        Some ( results ) => {
            let mut found: bool = false;
            //println!("....# of intervention details: {}", tmp_ivec.clone().iter().len());
            for item in results{
                //println!("....check item.id {}={}", insert_ivdtls_results, item.id);
                if item.id == tmp_id {
                    //println!("....matched.");
                    found = true;
                    new_intv_dtls = item.clone();
                }
            }
            assert!( found );  // if the id was not found, the insert failed
        },
        None => {
            assert!(false);
        }
    }

    //Step 3: update again
    frm.intervention_details_id = new_intv_dtls.id.to_string();
    frm.type_id = "10".to_string();
    frm.value = "Detected".to_string();
    frm.notes = "Updated now".to_string();

    //Step 1: create a record
    let insert_ivdtls_results = idao.upsert_intervention_details_from_intv_form(frm.clone(), test_user_id).await.unwrap();
    if insert_ivdtls_results == constants::NOT_SPECIFIED_ID{
        assert!(false); // did not receive a new id
    }
    else{ 
        tmp_id = insert_ivdtls_results; // returns the indet
        assert!(true);
    }

    //Step 4: get it back and validate the changes
    let qry_results = idao.get_all_intervention_details_for_an_intervention(intervention_id, constants::NOT_SPECIFIED_ID).await;
    let tmp_ivec = qry_results.unwrap().clone();
    match tmp_ivec.clone(){
        Some ( results ) => {
            let mut found: bool = false;
            //println!("....# of intervention details: {}", tmp_ivec.clone().iter().len());
            for item in results{
                //println!("....check item.id {}={}", insert_ivdtls_results, item.id);
                if item.id == tmp_id {
                    //println!("....matched.");
                    found = true;
                    //new_intv_dtls = item.clone();
                }
            }
            assert!( found );  // if the id was not found, the insert failed
        },
        None => {
            assert!(false);
        }
    }
}

///
/// Tests the ability for the DAO to SELECT Intervention records
///  * InterventionDAO::new()
///  * get_intervention()
/// 
#[tokio::test]
async fn test_get_intervention(){
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
            Err(e) => {
            tracing::warn!("{}", e);
            panic!("{}", e)
        },
    };

    let temp_intv = {InterventionDAO::new( db_pool.clone() ).await}.get_intervention( 1 ).await.unwrap();
    let results =  match temp_intv {
        Some(i) => {
            let tmp_scheduled_timestamp = match NaiveDateTime::parse_from_str(&"2026-08-08 12:05:00".to_string(), &"%Y-%m-%d %H:%M:%S".to_string()){
                Ok(result) => Some(result),
                Err(e) => {
                    println!("Error parsing datetime: {}", e);
                    Some(Utc::now().naive_utc())
                }
            };

            let tmp_intv = Intervention {
                id: 1, // intervention_id: i64 = rng.random();
                encounter_id: 1, // let encounter_id: i64 = rng.random();
                description: "broken foot from tree climbing".to_string(),
                notes: "".to_string(),
                location_id: 9,
                users_id: 2,
                intervention_type_id: 100038,
                status_id: 19,
                status_code: "Admit".to_string(),
                intervention_type: "Procedure: Collect Vitals".to_string(),
                room_identifier: "Exam Room 1".to_string(),
                scheduled_timestamp: tmp_scheduled_timestamp, 
                performed_timestamp: tmp_scheduled_timestamp
            };

            // if the key fields match, the DAO has successfully pulled the right record.
            // Some fields are subject to frequent change and not worth testing.
            assert!(tmp_intv.id == i.id, "ids do not match");
            assert!(tmp_intv.encounter_id == i.encounter_id, "encounter_id do not match");
            assert!(tmp_intv.description == i.description , "description do not match");
            assert!(tmp_intv.notes == i.notes , "notes do not match");
            assert!(tmp_intv.location_id == i.location_id , "location_id do not match");
            assert!(tmp_intv.users_id == i.users_id , "users_id do not match");
            assert!(tmp_intv.intervention_type_id == i.intervention_type_id , "intervention_type_id do not match");
            assert!(tmp_intv.status_id == i.status_id, "status_id do not match");
            true
        },
        None => {
            assert!(false, "No intervention was returned for the test");
            false
        },
    } ;
    assert!(results)
}

///
/// Tests the ability for the DAO to SELECT Interventions records
///  * InterventionDAO::new()
///  * get_interventions()
/// 
#[tokio::test]
async fn test_get_interventions_plural(){
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
        tracing::warn!("{}", e);
        panic!("{}", e)
        },
    };

    let idao = InterventionDAO::new( db_pool.clone() ).await;

    // ensure the calls are valid and return a result. Inspection of every field is too much
    let temp_intv = idao.get_interventions( 1, true ).await.unwrap();
    match temp_intv {
        Some(i) => assert!(i.iter().count() == 1, "More or less than 1 intervention returned from InterventionDAO::get_interventions(, true)"),
        None => assert!(false, "No intervention was returned for the test"),
    };

    // ensure the calls are valid and return a result. Inspection of every field is too much
    let temp_obj = idao.get_interventions( 1, false ).await.unwrap();
    match temp_obj {
        Some(i) => assert!(i.iter().count() != 1, "Expected more than 1 intervention to be returned from InterventionDAO::get_interventions(n, false)"),
        None =>  assert!(false, "Expected more than 1 intervention to be returned from InterventionDAO::get_interventions(n, false). No intervention was returned."),
    };
}

///
/// Tests the ability for the DAO to SELECT the most recent vitals-Intervention record
///  * InterventionDAO::new()
///  * get_most_recent_vitals()
/// 
#[tokio::test]
async fn test_get_most_recent_vitals(){
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::warn!("{}", e);
            panic!("{}", e)
        },
    };

    let temp_obj = {InterventionDAO::new( db_pool.clone() ).await}.get_most_recent_vitals( 4 ).await.unwrap();
    let results =  match temp_obj {
        Some(i) => {
            match i.id {
                4 => true, // Encounter id=4 and Intervention id=4
                _ => false,
            }
        },
        None => false,
    } ;
    assert!(results)
}

///
/// Tests the ability for the DAO to INSERT or UPDATE an Intervention Details record
///  * InterventionDAO::new()
///  * upsert_intervention_details_from_intv_form()
/// 
#[tokio::test]
async fn test_upsert_intervention_from_intv_form(){
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::warn!("{}", e);
            panic!("{}", e)
        },
    };

    let tmp_frm = InterventionDetailsDataForm {
        intervention_details_id: constants::NOT_SPECIFIED_ID.to_string(),
        intervention_id: "1".to_string(),
        type_id: "1".to_string(),
        form_errors: "UNIT TEST RECORD".to_string(),
         ..Default::default() 
    };

    // if the key fields match, the DAO has successfully pulled the right record.
    // Some fields are subject to frequent change and not worth testing.
    let obj_id = {InterventionDAO::new( db_pool.clone() ).await}.upsert_intervention_details_from_intv_form( tmp_frm, constants::INVALID_OTHER_ID ).await.unwrap();
    assert!(obj_id != constants::INVALID_OTHER_ID, "New ID was not returned, update did not occur");
}