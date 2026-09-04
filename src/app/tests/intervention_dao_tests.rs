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

use maple_emr::dao::intervention_dao::InterventionDAO;

use sqlx::postgres::{PgPoolOptions}; 
use tracing;

use maple_emr::constants;
use maple_emr::ui::data_forms::*;
use maple_emr::dto::intervention_detail::InterventionDetail;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

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