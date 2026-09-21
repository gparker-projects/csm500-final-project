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
use chrono::{NaiveDate, Utc};

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
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            println!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };

    let test_user_id = 2;

    // Test 1: instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
    // does not instantiate however, the remainder of this test will fail.
    let pdao = PatientDAO::new( db_pool.clone() );

    // Test 2: Valid user_id test
    let qry_results: Option< Vec<Patient>> = pdao.await.get_patients_at_users_site_no_discharge(test_user_id).await.unwrap();
    match qry_results{
        Some (patient_list) => {
            println!("Retrieved {} patients", patient_list.len());
            assert!(patient_list.len() > 0); // some patients must have been returned; # will vary with testing

            for p in patient_list{ // this ends up being a bit of a data validation
                assert!(p.id != constants::INVALID_PATIENT_ID, "Patient id=-1 returned, patients are not valid");
                assert!(p.encounter_id != constants::INVALID_OTHER_ID, "Encounter id=-1 returned, patients are not valid");
                assert!(p.location_id != constants::INVALID_OTHER_ID, "Location id=-1 returned, patients are not valid");

                assert!(p.phn <= 9999999999, "Non-10 digit PHN returned, patients are not valid");
                assert!(p.phn >  8999999999, "Lower than allowed value PHN returned, patients are not valid");
                assert!(p.legal_first_name.len() > 0, "Empty First Name found, patients are not valid");
                assert!(p.legal_last_name.len() > 0, "Empty Last Name found, patients are not valid");

                assert!(p.legal_first_name.len() < 200, "First Name too long found, patients are not valid");
                assert!(p.legal_last_name.len() < 200, "Last Name too long found, patients are not valid");
                assert!(p.legal_middle_names.len() < 200, "Middle Name too long found, patients are not valid");

                assert!(p.admit_notes.len() < 2000, "Admit notes too long found, patients are not valid");
                assert!(p.discharge_notes.len() < 2000, "Discharge notes too long found, patients are not valid");

                assert!(Some(p.admit_timestamp).is_some(), "Empty Admit Timestamp found, patients are not valid");

                // check admit/discharge are after this course?
                let earliest_birth_date = NaiveDate::from_ymd_opt(1880, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(); // clean date that will capture 100% of all living people
                let earliest_record_date = NaiveDate::from_ymd_opt(2026, 7, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(); // July 1 of this year, approximate start date of CSM500
          
                assert!(p.birth_date >= earliest_birth_date, "Birth date predates oldest person alive, patients are not valid");
                assert!(p.birth_date <= Utc::now().naive_utc(), "Birth date is in the future, patients are not valid");

                assert!(p.admit_timestamp >= earliest_record_date, "Admit Timestamp predates system creation, patients are not valid");
                assert!(p.admit_timestamp <= Utc::now().naive_utc(), "Admit Timestamp is in the future, patients are not valid");

                match p.discharge_timestamp {
                    Some( dt ) => {
                        assert!(dt >= earliest_record_date, "Discharge Timestamp predates system creation, patients are not valid");
                        assert!(dt <= Utc::now().naive_utc(), "Discharge Timestamp is in the future, patients are not valid");
                    },
                    None => {
                        assert!(p.discharge_notes.len() == 0, "Discharge Notes present, Discharge Timestamp is not; patients are not valid");
                    },
                };
            }
        }
        None => {
            println!("No patients found for user_id={}", test_user_id);
            assert!(false);
        }
    }

    // Test 3: Invalid user_id test
    let qry_results: Option< Vec<Patient>> = {PatientDAO::new( db_pool.clone() )}.await.get_patients_at_users_site_no_discharge(constants::INVALID_OTHER_ID).await.unwrap();
    match qry_results{
        Some (_items) => {
            if _items.len() > 0 {
                assert!(false, "Invalid user_id (-1), no results should have been returned")
            }
            else{  // method returns an empty vector, so as long as length is 0, this is okay
                assert!(true); 
            }
        },
        None => assert!(true),
    };
}

/// ### test_get_patient_details()
/// 
/// Tests the ability for the DAO to retrieve Patient details
/// 
///  If there are no valid patients, you may need this SQL: update encounter set discharge_timestamp = null where id = 1
/// 
///   Specifically tests: PatientDAO::get_patient_details() 
///
#[tokio::test]
async fn test_get_patient_details() {
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            println!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };
    let test_user_id = 2;
    let test_patient_id_discharged = 1; // patient 1 has been discharged and should be preserved for testing
    let test_patient_id_not_discharged = 27; // patient 27 has NOT been discharged and should be preserved for testing

    // Test 1: New - Instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
    // does not instantiate however, the remainder of this test will fail.
    let pdao = PatientDAO::new( db_pool.clone() ).await;

    // Test 2: Valid discharge
    let qry_results: Option<Patient> = pdao.get_patient_details_optional_discharged(test_user_id, test_patient_id_discharged, false).await.unwrap();
    match qry_results{
        Some (_p) => {
            assert!(true);
        }
        None => {
            println!("Test 1 (Discharged): Patient expected, no patient returned for id={} users_id={}", test_patient_id_discharged, test_user_id);
            assert!( false );
        }
    }

    // Test 3: Retrieve details again of discharged patient
    let qry_results: Option<Patient> = {PatientDAO::new( db_pool.clone() ).await}.get_patient_details_not_discharged(test_user_id, test_patient_id_not_discharged).await.unwrap();
    match qry_results{
        Some (_p) => {
            assert!(true);
        }
        None => {
            println!("Test 2 (Not Discharged): Patient expected, no patient returned for id={} users_id={}", test_patient_id_not_discharged, test_user_id);
            assert!( false );
        }
    }

    // Test 4: InValid discharge
    let qry_results: Option<Patient> = pdao.get_patient_details_optional_discharged(test_user_id, constants::INVALID_OTHER_ID, false).await.unwrap();
    match qry_results{
        Some (_p) => {
            
            assert!( false,"Patient was returned, when none expected for id={} users_id={}", test_patient_id_discharged, test_user_id);
        }
        None => {
           assert!(true);
        }
    }
}


/// ### test_wrapper_patient_dao()
/// 
/// Calls subordindate tests that MUST be executed in a specific order
///
#[tokio::test]
async fn   test_wrapper_patient_dao() {

    // these both create new encounters and during parallel thread execution mess up the discharge
    // which is determistic on its ID.

    println!(">> Stage 1: test_upsert_patient_from_admit_form()");
    test_upsert_patient_from_admit_form().await;

    println!(">> Stage 2: test_upsert_encounter_from_admit_form()");
    test_upsert_encounter_from_admit_form().await;

    // must perform discharge last, otherwise the other items running in parallel mess up the id sequencing
    println!(">> Stage 3: test_update_encounter_from_discharge_form()");
    test_update_encounter_from_discharge_form().await;
}

/// ### test_upsert_patient_from_admit_form()
/// 
/// Tests the ability for the DAO to insert/update an encounter, based on an admit form
/// 
///   Specifically tests: PatientDAO::upsert_patient_from_admit_form() 
/// 
/// DO NOT ENABLE: \[tokio::test] HERE... it must be controlled by the wrapper test
async fn test_upsert_patient_from_admit_form() {
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            println!("{}", e);
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
        birthdate: DataGenerator::get_date_as_yyyy_mon_dd(),
        encounter_id: constants::INVALID_OTHER_ID.to_string(), // this field and others are not actually set/used by upsert_patient_from_admit_form() 
        location_id: constants::INVALID_OTHER_ID.to_string(), // will be ignored
        action_flag: "Y".to_string(), // will be ignored
        admit_notes: DataGenerator::get_lorem_ipsum(100), // will be ignored
        form_errors: "".to_string(), // will be ignored
        user_prompt: DataGenerator::get_lorem_ipsum(100)  // will be ignored
    };

    let pdao = PatientDAO::new( db_pool ).await;
    
    // Test 1: Upsert a Patient with a valid id
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
            println!("Patient id={} expected, no patient returned: {}", test_patient_id, e);
            assert!(false)
        }
    }

    // Test 2: Upsert a Patient with invalid data
    let mut tmp_frm2 = tmp_frm.clone();
    tmp_frm2.birthdate = "INVALID UNIT TEST BIRTHDATE".to_string(); // this is intentionally invalid

    let patient_results = pdao.upsert_patient_from_admit_form(tmp_frm2.clone(), test_user_id).await;
    match patient_results {
        Ok ( item ) => {
            if item != constants::INVALID_OTHER_ID {
                assert!(false, "Results returned when not expected; Invalid Patient id (-1)");
            }            
        },
        Err( _ ) => assert!(true),
    }
}


/// ### test_upsert_encounter_from_admit_form()
/// 
/// Tests the ability for the DAO to insert/update an encounter, based on an admit form
/// 
///   Specifically tests: PatientDAO::upsert_encounter_from_admit_form() 
/// 
/// DO NOT ENABLE: \[tokio::test] HERE... it must be controlled by the wrapper test
async fn test_upsert_encounter_from_admit_form() {
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            println!("{}", e);
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
        birthdate: DataGenerator::get_date_as_yyyy_mon_dd().to_string(),
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

    // Test 3: perform an invalid update
    let mut tmp_frm2 = tmp_frm.clone();
    tmp_frm2.location_id = "TEST".to_string(); // non-standard invalid id

    let results = pdao.upsert_encounter_from_admit_form(tmp_frm2.clone(), test_user_id).await;
    match results {
        Ok ( item ) => {
            if item != constants::INVALID_OTHER_ID {
                println!("found id={}", item);
                assert!(false, "Results returned when not expected; Invalid data was provided");
            }            
        },
        Err( _ ) => assert!(true),
    }
}

/// ### test_update_encounter_from_discharge_form()
/// 
/// Tests the ability for the DAO to update an encounter, based on a discharge form
/// 
///   Specifically tests: PatientDAO::update_encounter_from_discharge_form() 
/// 
/// DO NOT ENABLE: \[tokio::test] HERE... it must be controlled by the wrapper test
async fn test_update_encounter_from_discharge_form() {
    let db_url = DB_CONN_STR;
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            println!("{}", e);
            assert!(false);
            panic!("{}", e)
        },
    };  // Done: setting up the connection for the DAO test

    let test_user_id: i64 = 2;
    let mut test_patient_id = 34;               // <-------------------------- these might need to be changed, if the data changes
    let mut tmp_encounter_id = 17;

    // start by getting the current patient and encounter ids
    let pdao = PatientDAO::new( db_pool ).await;

    // Test 1: Query the data
    let qry_results: Option<Patient> = pdao.get_patient_details_not_discharged(test_user_id, test_patient_id).await.unwrap();
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
        discharge_notes: DataGenerator::get_lorem_ipsum(200)
    };

    // Test 2: Update the encounter using the discharge form; starts by creating a new encounter (id = -1)
    let results = pdao.update_encounter_from_discharge_form(tmp_frm.clone(), test_user_id).await;
    match results {
        Ok ( enc_id ) => {
            if enc_id != constants::INVALID_OTHER_ID{

                // if the update actually worked, the data should have changed
                let qry_results: Option<Patient> = pdao.get_patient_details_optional_discharged(test_user_id, test_patient_id, false).await.unwrap();
                match qry_results{
                    Some (p) => {
                        assert_eq!( p.discharge_notes, tmp_frm.discharge_notes );        // discharge notes should be the same as what was sent in
                        assert_ne!( p.discharge_timestamp.unwrap(), p.admit_timestamp ); // update timestamp should be different
                        assert!(true)
                    }
                    None => {
                        println!("Patient expected, no patient returned");
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
            println!("Error encountered: {}", e);
            assert!(false)
        }
    }

    // Test 3: Test to throw an error within the  DAO call
    let mut tmp_frm2 = tmp_frm.clone(); 
    tmp_frm2.encounter_id = "INVALID UNIT TEST PATIENT ID".to_string(); // intentionally will cause the DAO to throw an error, proving it is handled
    tmp_frm2.discharge_notes = "Updated by Unit Test: test_update_encounter_from_discharge_form(), Test 3".to_string(); 

    let results2 = pdao.update_encounter_from_discharge_form(tmp_frm2.clone(), test_user_id).await;
    match results2 {
        Ok ( enc_id ) => {
            if enc_id != constants::INVALID_OTHER_ID {
                assert!(false, "Unexpected Encounter id returned; invalid data was provided")
            }
            else{
                assert!(true)
            }            
        },
        Err (_) => assert!(true),
    };
}

