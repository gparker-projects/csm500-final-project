///
/// #Unit & Integration tests for the PatientDAO module
/// 
/// * new()
/// * get_patient_details
/// * update_encounter_from_discharge_form
/// * upsert_encounter_from_admit_form
/// * upsert_patient_from_admit_form
/// * get_patients_at_users_site_no_discharge
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 

mod common;

use sqlx::postgres::{PgPoolOptions}; 
use tracing;

use maple_emr::constants;
use maple_emr::dao::patient_dao::PatientDAO;
use maple_emr::dto::patient::Patient;
use maple_emr::ui::data_forms::*;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

use common::test_utils::DataGenerator;

#[cfg(test)]

/// ### test_get_patients_at_users_site_no_discharge()
/// 
/// Tests the ability for the DAO to retrieve Patients
/// 
///   Specifically tests: PatientDAO::test_get_patients_at_users_site_no_discharge() 
///
#[tokio::test]
async fn test_get_patients_at_users_site_no_discharge() {
    let test_user_id = 2;
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::debug!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };

  // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
  // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
  // does not instantiate however, the remainder of this test will fail.
  let pdao = PatientDAO::new( db_pool );
  let qry_results: Option< Vec<Patient>> = pdao.await.get_patients_at_users_site_no_discharge(test_user_id).await.unwrap();
  
  match qry_results{
      Some (patient_list) => {
        tracing::debug!("Retrieved {} patients", patient_list.len());
        assert!(patient_list.len() > 7);
      }
      None => {
        tracing::debug!("No patients");
        assert!(false);
      }
  }
}

/// ### test_get_patient_details()
/// 
/// Tests the ability for the DAO to retrieve Patient details
/// 
///   Specifically tests: PatientDAO::get_patient_details() 
///
#[tokio::test]
async fn test_get_patient_details() {
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::debug!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };
    let test_user_id = 2;
    let test_patient_id = 1;

    // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
    // does not instantiate however, the remainder of this test will fail.
    let pdao = PatientDAO::new( db_pool ).await;
    let qry_results: Option<Patient> = pdao.get_patient_details(test_user_id, test_patient_id).await.unwrap();
    match qry_results{
        Some (_p) => {
            assert!(true);
        }
        None => {
            tracing::debug!("Patient expected, no patient returned");
            assert!(false);
        }
    }
}

#[tokio::test]
/// ### test_wrapper_patient_dao()
/// 
/// Calls subordindate tests that MUST be executed in a specific order
///
async fn test_wrapper_patient_dao() {

    // these both create new encounters and during parallel thread execution mess up the discharge
    // which is determistic on its ID.
    test_upsert_patient_from_admit_form().await;
    test_upsert_encounter_from_admit_form().await;

    // must perform discharge last, otherwise the other items running in parallel mess up the id sequencing
    test_update_encounter_from_discharge_form().await;
}

/// ### test_upsert_patient_from_admit_form()
/// 
/// Tests the ability for the DAO to insert/update an encounter, based on an admit form
/// 
///   Specifically tests: PatientDAO::upsert_patient_from_admit_form() 
///
async fn test_upsert_patient_from_admit_form() {
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::debug!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };  // Done: setting up the connection for the DAO test
    let test_user_id = 2;
    let test_patient_id = 34;               // <-------------------------- these might need to be changed, if the data changes

    let tmp_frm = AdmitDataForm{
        patient_id: test_patient_id.to_string(), // <-------------------------- these might need to be changed, if the data changes
        phn: "9962692135".to_string(),           // <--------------------------
        patient_first_name: DataGenerator::get_first_name(100),
        patient_last_name: DataGenerator::get_first_name(100),
        patient_middle_name: DataGenerator::get_first_name(100),
        birthdate: DataGenerator::get_date().to_string(),
        encounter_id: constants::INVALID_OTHER_ID.to_string(), // this field and others are not actually set/used by upsert_patient_from_admit_form() 
        location_id: constants::INVALID_OTHER_ID.to_string(), // will be ignored
        action_flag: "Y".to_string(), // will be ignored
        admit_notes: DataGenerator::get_lorem_ipsum(100), // will be ignored
        form_errors: "".to_string(), // will be ignored
        user_prompt: DataGenerator::get_lorem_ipsum(100)  // will be ignored
    };

    let pdao = PatientDAO::new( db_pool ).await;
    let patient_results = pdao.upsert_patient_from_admit_form(tmp_frm.clone(), test_user_id).await;
    match patient_results {
        Ok ( p_id ) => {
            if p_id == test_patient_id{
                assert!(true)
            }
            else{
                println!("Patient id={} expected, different patient (id={}) returned", p_id, test_patient_id);
                assert!(false)
            }
        }
        Err(e) => {
            println!("Patient expected, no patient returned: {}", e);
            assert!(false)
        }
    }
}


/// ### test_upsert_encounter_from_admit_form()
/// 
/// Tests the ability for the DAO to insert/update an encounter, based on an admit form
/// 
///   Specifically tests: PatientDAO::upsert_encounter_from_admit_form() 
///
async fn test_upsert_encounter_from_admit_form() {
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::debug!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };  // Done: setting up the connection for the DAO test

    let test_user_id = 2;
    let test_patient_id = 34;               // <-------------------------- these might need to be changed, if the data changes
    let mut tmp_encounter_id = constants::INVALID_OTHER_ID;

    let mut tmp_frm = AdmitDataForm{
        patient_id: test_patient_id.to_string(), // <-------------------------- these might need to be changed, if the data changes
        phn: "9962692135".to_string(),           // <--------------------------
        patient_first_name: DataGenerator::get_first_name(100),
        patient_last_name: DataGenerator::get_first_name(100),
        patient_middle_name: DataGenerator::get_first_name(100),
        birthdate: DataGenerator::get_date().to_string(),
        encounter_id: constants::INVALID_OTHER_ID.to_string(), // this field and others are not actually set/used by upsert_patient_from_admit_form() 
        location_id: "12".to_string(), // will be ignored
        action_flag: "Y".to_string(), // will be ignored
        admit_notes: DataGenerator::get_lorem_ipsum(100), // will be ignored
        form_errors: "".to_string(), // will be ignored
        user_prompt: DataGenerator::get_lorem_ipsum(100)  // will be ignored
    };

    let pdao = PatientDAO::new( db_pool ).await;

    // first create a new encounter (id = -1)
    let results = pdao.upsert_encounter_from_admit_form(tmp_frm.clone(), test_user_id).await;
    match results {
        Ok ( enc_id ) => {
            if enc_id != constants::INVALID_OTHER_ID{
                tmp_encounter_id = enc_id;
                assert!(true)
            }
            else{
                println!("New encounter id expected, (id=-1) returned");
                assert!(false)
            }
        }
        Err(e) => {
            println!("Error encounterred: {}", e);
            assert!(false)
        }
    }

    tmp_frm.encounter_id = tmp_encounter_id.to_string();
    tmp_frm.admit_notes = "UPDATED as part of unit testing".to_string();

    // update encounter using id obtained
    let results = pdao.upsert_encounter_from_admit_form(tmp_frm.clone(), test_user_id).await;
    match results {
        Ok ( enc_id ) => {
            if enc_id != constants::INVALID_OTHER_ID{
                assert!(true)
            }
            else{
                println!("Encounter id=-1 returned");
                assert!(false)
            }
        }
        Err(e) => {
            println!("Error encounterred: {}", e);
            assert!(false)
        }
    }
}

/// ### test_update_encounter_from_discharge_form()
/// 
/// Tests the ability for the DAO to update an encounter, based on a discharge form
/// 
///   Specifically tests: PatientDAO::update_encounter_from_discharge_form() 
///
async fn test_update_encounter_from_discharge_form() {
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            tracing::debug!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };  // Done: setting up the connection for the DAO test

    let test_user_id: i64 = 2;
    let mut test_patient_id = 34;               // <-------------------------- these might need to be changed, if the data changes
    let mut tmp_encounter_id = 17;

    // start by getting the current patient and encounter ids
    let pdao = PatientDAO::new( db_pool ).await;
    let qry_results: Option<Patient> = pdao.get_patient_details(test_user_id, test_patient_id).await.unwrap();
    match qry_results{
        Some (p) => {
            test_patient_id = p.id;
            tmp_encounter_id = p.encounter_id;
        }
        None => {
            assert!(false)
        }
    }

    let tmp_frm = DischargeDataForm{
        patient_id: test_patient_id.to_string(), // <-------------------------- these might need to be changed, if the data changes
        encounter_id: tmp_encounter_id.to_string(),                      // this field and others are not actually set/used by upsert_patient_from_admit_form() 
        discharge_notes: DataGenerator::get_lorem_ipsum(100)
    };

    // first create a new encounter (id = -1)
    let results = pdao.update_encounter_from_discharge_form(tmp_frm.clone(), test_user_id).await;
    match results {
        Ok ( enc_id ) => {
            if enc_id != constants::INVALID_OTHER_ID{

                // if the update actually worked, the data should have changed
                let qry_results: Option<Patient> = pdao.get_patient_details(test_user_id, test_patient_id).await.unwrap();
                match qry_results{
                    Some (p) => {
                        assert_eq!( p.discharge_notes, tmp_frm.discharge_notes );        // discharge notes should be the same as what was sent in
                        assert_ne!( p.discharge_timestamp.unwrap(), p.admit_timestamp ); // update timestamp should be different
                        assert!(true)
                    }
                    None => {
                        tracing::debug!("Patient expected, no patient returned");
                        assert!(false)
                    }
                }
            }
            else{
                println!("New encounter id expected, (id=-1) returned");
                assert!(false)
            }
        }
        Err(e) => {
            println!("Error encounterred: {}", e);
            assert!(false)
        }
    }
}

