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
///
/// Tests the ability for the DAO to retrieve Patients
///
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


///
/// Tests the ability for the DAO to retrieve Patient details
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
    let qry_results: Option< Patient> = pdao.get_patient_details(test_user_id, test_patient_id).await.unwrap();
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

///
/// Tests the ability for the DAO to insert/update an encounter, based on an admit form
///
#[tokio::test]
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

///
/// Tests the ability for the DAO to insert/update an encounter, based on an admit form
///
#[tokio::test]
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

/*
    UPDATE encounter
        SET admit_notes = '{admit_notes}',
            discharge_timestamp = to_timestamp('{discharge_timestamp}', 'YYYY/MM/DD HH24:MI:SS'),
            discharge_notes = '{discharge_notes}',
            patient_id = {patient_id},
            location_id = {location_id}
        WHERE id = {encounter_id} RETURNING ID;
*/
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

///
/// Tests the ability for the DAO to update an encounter, based on a discharge form
///
#[tokio::test]
async fn test_update_encounter_from_discharge_form() {

}

