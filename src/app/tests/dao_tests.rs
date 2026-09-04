///
/// Unit & Integration tests for the DAO module
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]

mod common;

use maple_emr::dao::intervention_dao::InterventionDAO;
//use common::test_utils::*; 

use sqlx::postgres::{PgPoolOptions}; 
use tracing;

use maple_emr::constants;
use maple_emr::ui::data_forms::*;
use maple_emr::dao::patient_dao::PatientDAO;
use maple_emr::dao::feature_preference_dao::FeaturePreferenceDAO;
use maple_emr::dto::feature_preference::FeaturePreference;
use maple_emr::dto::intervention_detail::InterventionDetail;

use maple_emr::dto::patient::Patient;



pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

///
/// Tests the ability for the DAO to retrieve Patients
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
          panic!("{}", e)
        },
    };

  // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
  let pdao = PatientDAO::new( db_pool );
  let qry_results: Option< Vec<Patient>> = pdao.await.get_patients_at_users_site_no_discharge(test_user_id, false).await.unwrap();
  
  match qry_results{
      Some (patient_list) => {
        tracing::debug!("Retrieved {} patients", patient_list.len());
        assert_eq!(patient_list.len(), 7);
      }
      None => {
        tracing::debug!("No patients");
        assert!(false);
      }
  }
}

///
/// Tests the ability for the DAO to CREATE, SELECT and UPDATE Intervention Detail records
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

    println!("Testing: upsert_intervention_details_from_intv_form(): first insertion");

    let idao = InterventionDAO::new( db_pool.clone() ).await; // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
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


  //Step 1: create a record
  //Step 2: get it back
  //Step 3: update
  //Step 4: get it back and validate the changes

///
/// Tests the ability for the DAO to CREATE, SELECT and UPDATE Feature Priority records
/// 
#[tokio::test]
async fn test_ins_get_upd_feature_priority() {
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

  let test_user_id = 5; // we will use user #5, which is admin user and has lots of data (permissions)
  let test_feature_id: i64 = 100002;
  let mut fp_id: i64 = constants::INVALID_OTHER_ID;

  println!("Testing: get_all_active_feature_preferences_for_user(): first insertion");

  // try to create a FeaturePreference
  let fdao = FeaturePreferenceDAO::new( db_pool.clone() ).await; // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
  let qry_results = fdao.upsert_feature_preference(test_user_id, test_feature_id).await;

  match qry_results.unwrap(){
      Some ( results ) => {
        fp_id = results;
        assert!(true);
      }
      None => {
        assert!(false);
      }
  }

  let mut new_fp: FeaturePreference = Default::default(); // use a dummy record to satify the compiler below

  println!("..retreive created record");
  // try to pull out that same that was created

  let qry_results = fdao.get_active_feature_preferences_for_user(test_user_id, test_feature_id, 5, false).await;
  match qry_results.unwrap(){
        Some ( results ) => {
            let mut found: bool = false;

            for item in results{
                println!("....check item.id {}={}", fp_id, item.id);
                
                if item.id == fp_id {
                    println!("....matched.");
                    found = true;
                    new_fp = item.clone();
                }
            }
            assert!(found);  // if the id was not found, the insert failed
        }
        None => {
            assert!(false);
        }
  }

  println!("..update created record");
  // try to update the FeaturePreference
  let qry_results = fdao.upsert_feature_preference(test_user_id, test_feature_id).await;
  match qry_results.unwrap(){
        Some ( _results ) => {
            assert!(true); // no magic here, it should just not fail
        }
        None => {
            assert!(false);
        }
  }

  println!("..retreive updated record");
  // try to pull out that same that was updated: this time, the updated timestamp should be different than the first time
  let qry_results_updated = fdao.get_active_feature_preferences_for_user(test_user_id,test_feature_id, 5, false).await;
  match qry_results_updated.unwrap(){
      Some ( results ) => {
        let mut updated: bool = false;

        for item in results{
           if item.id == fp_id {
              if new_fp.calculation_date != item.calculation_date {
                  println!("{} != {}", new_fp.calculation_date.to_string(), item.calculation_date.to_string() );
                  updated = true;
              }
           }
        }
        assert!(updated);  // if the id was not found, the insert failed
      }
      None => {
        assert!(false);
      }
  }
}