///
/// Unit & Integration tests for the FeaturePreferenceDAO module. Includes
/// 
/// * new()
/// * get_active_feature_preferences_for_user
/// * get_active_feature_preferences_of_interventions_for_user
/// * get_active_feature_preferences_of_intervention_details_for_user
/// * upsert_feature_preference 
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]

mod common;

use sqlx::postgres::{PgPoolOptions}; 
use tracing;

use maple_emr::constants;
use maple_emr::dao::feature_preference_dao::FeaturePreferenceDAO;
use maple_emr::dto::feature_preference::FeaturePreference;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

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

  // check we can get the Intervention-level feature preference back
  let qry_results = fdao.get_active_feature_preferences_for_user(test_user_id, test_feature_id, 5, true).await;
  match qry_results.unwrap(){
        Some ( results ) => {
            let mut found: bool = false;

            println!("....results.len()={}", results.len());

            for item in results{
                println!("....check item.id {}={}", fp_id, item.id);
                
                if item.id == fp_id {
                    println!("....matched.");
                    found = true;
                    new_fp = item.clone();
                }
            }
            assert!( found );  // if the id was not found, the insert failed
        }
        None => {
            assert!( false );
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
  let qry_results_updated = fdao.get_active_feature_preferences_for_user(test_user_id,test_feature_id, 5, true).await;
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