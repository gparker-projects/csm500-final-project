///
/// Unit & Integration tests for the FeaturePreference DTO and FeaturePreferenceDAO module. Includes:
/// 
/// * new()
/// * get_active_feature_preferences_for_user
/// * get_active_feature_preferences_of_interventions_for_user
/// * get_active_feature_preferences_of_intervention_details_for_user
/// 
/// * upsert_feature_preference 
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
/// 
#[cfg(test)]

mod common;

use sqlx::postgres::{PgPoolOptions}; 
use chrono::NaiveDate;

use maple_hms::constants;
use maple_hms::dao::feature_preference_dao::FeaturePreferenceDAO;
use maple_hms::dto::feature_preference::FeaturePreference;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

/// ### test_ins_get_upd_feature_priority_intv_level()
/// 
/// Tests the ability for the FeaturePreference::new() to correctly instantiate and populate a new object.
/// Specifically tests:
///  * get_unique_key() and data fields only
/// 
#[tokio::test]
async fn test_ins_get_upd_feature_priority_intv_level() {

    let earliest_birth_date = NaiveDate::from_ymd_opt(1880, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(); // clean date that will capture 100% of all living people

    let tmp_calculation_date = earliest_birth_date;

    let fp: FeaturePreference = FeaturePreference::new(
        constants::INVALID_OTHER_ID,
        10, 
        11,        
        tmp_calculation_date, 
        constants::INVALID_OTHER_ID,
        constants::INVALID_OTHER_ID,
        constants::INVALID_OTHER_ID,
        constants::INVALID_OTHER_ID,
        "UNIT TEST FAKE VALUE".to_string()
    );

    assert_eq!(fp.id, constants::INVALID_OTHER_ID);
    assert_eq!(fp.display_order, 10);
    assert_eq!(fp.weight, 11);
    assert_eq!(fp.calculation_date, tmp_calculation_date);
    assert_eq!(fp.users_id, constants::INVALID_OTHER_ID);
    assert_eq!(fp.department_id, constants::INVALID_OTHER_ID);
    assert_eq!(fp.feature_id, constants::INVALID_OTHER_ID);
    assert_eq!(fp.ref_group_id, constants::INVALID_OTHER_ID);
    assert_eq!(fp.ref_name, "UNIT TEST FAKE VALUE".to_string());
    assert_eq!(fp.get_unique_key(), "-1.-1".to_string());
}

/// ### test_upsert_get_feature_preferences_of_interventions_for_user()
/// 
/// Tests the ability for the DAO to CREATE, SELECT and UPDATE Feature Priority records at the Intervention level (not details)
/// Specifically tests:
///  * upsert_feature_preference
///  * get_active_feature_preferences_of_interventions_for_user
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

#[tokio::test]
async fn test_upsert_get_feature_preferences_of_interventions_for_user() {
  let db_pool: sqlx::Pool<sqlx::Postgres> = match PgPoolOptions::new()
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

  let test_user_id = 2; // we will use user #2, which is admin user and has lots of data (permissions)
  let test_feature_id: i64 = 100002;
  let mut fp_id: i64 = constants::INVALID_OTHER_ID;
  let test_upper_limit = 20;

  println!("Testing: get_all_active_feature_preferences_for_user(): first insertion");

  // try to create a FeaturePreference
  // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
  let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.upsert_feature_preference(test_user_id, test_feature_id).await;
  match qry_results.unwrap(){
      Some ( results ) => {
          fp_id = results; // record should have been created or updated
          assert!(true);
      }
      None => {
        assert!(false); // record should have been created or updated, if not, fail
      }
  }

  let mut new_fp: FeaturePreference = Default::default(); // use a dummy record to satify the compiler below

  println!("..first get: test_user_id={} test_feature_id={} 5 true", test_user_id, test_feature_id);

  // check we can get the Intervention-level feature preference back
  let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.get_active_feature_preferences_of_interventions_for_user(test_user_id, test_upper_limit).await;
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

  println!("..update created record: test_user_id={} test_feature_id={} 5 true", test_user_id, test_feature_id);
  // try to update the FeaturePreference
  let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.upsert_feature_preference(test_user_id, test_feature_id).await;
  match qry_results.unwrap(){
        Some ( _results ) => {
            assert!(true); // no magic here, it should just not fail
        }
        None => {
            assert!(false);
        }
  }

  println!("..retrieve updated: test_user_id={} test_feature_id={}, 5 true", test_user_id, test_feature_id); // NOTE: use of constants::CRT_ANY_INTERVENTION_GROUP below, it is a group for intervention-level test ids

  // try to pull out that same that was updated: this time, the updated timestamp should be different than the first time
  let qry_results_updated = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.get_active_feature_preferences_of_interventions_for_user(test_user_id, test_upper_limit).await;
  match qry_results_updated.unwrap(){
      Some ( results ) => {
        let mut updated: bool = false;
        println!("..Some() results, checking date was recalculated");

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
        println!("..Empty results");
        assert!(false);
      }
  }
}

/// ### test_get_preferences_for_new_never_existed_user()
/// 
/// Tests the FeaturePreference::get_active_feature_preferences_for_user_intervention_level() function via 
///   get_active_feature_preferences_of_interventions_for_user(). The function can block some calls and 
///   this test will target abnormal, expected situations.
/// 
/// Specifically tests:
///  * FeaturePreference::new()
///  * get_active_feature_preferences_for_user_intervention_level
/// 
#[tokio::test]
async fn test_get_preferences_for_new_never_existed_user() {
    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_CONN_STR)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            panic!("{}", e)
        },
    };

    let never_existed_user_id = 11999999999911; // this user has not/will not exist for the purposes of this course and as such will always have no feature preferences
    let mut test_upper_limit = 5;

    // Test 1a: check for preferences for a user who has never used it before
    let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.get_active_feature_preferences_of_interventions_for_user(never_existed_user_id, test_upper_limit).await;
    match qry_results.unwrap(){
        Some ( _results ) => assert!( false, "Test 1a failed" ),
        None => assert!( true ),
    } ;

    // Test 1b: check for preferences for an non-existant user with no permissions
    test_upper_limit = 1;

    let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.get_active_feature_preferences_of_interventions_for_user(never_existed_user_id, test_upper_limit).await;
    match qry_results.unwrap(){
        Some ( _results ) => assert!( false, "Test 1b failed" ),
        None => assert!( true ),
    } ;

    // Test 1c: check for preferences for an existing user with very few permissions
    let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.get_active_feature_preferences_of_interventions_for_user(3, test_upper_limit).await;
    match qry_results.unwrap(){
        Some ( _results ) => assert!( true ), // this user had permissions; the check should succeed, with records returned
        None => assert!( true, "Test 1b failed: records should have been returned" ),
    } ;

    // try to create a FeaturePreference that has never existed
    // both test user and feature must exist, but not have been used unfortunately
    //
    // NOTE: might need to use this prior to run: delete from feature_preference where users_id = 7 

    // delete the preferences for user_id, so no collisions
    delete_preference().await;

    let test_feature_id: i64 = 100002;
    let test_dr_drake_ramoray_user_id: i64 = 7;

    // Test 2: Valid Insertion
    let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.upsert_feature_preference(test_dr_drake_ramoray_user_id, test_feature_id).await;
    match qry_results.unwrap(){
        Some ( _results ) => assert!(true , "Test 2 failed" ), // record should have been created or updated
        None => assert!(false), // record should have been created or updated, if not, fail
    }

    // Test 2b: Invalid Feature Id Insertion
    let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.upsert_feature_preference(test_dr_drake_ramoray_user_id, constants::INVALID_OTHER_ID).await;
    match qry_results{
        Ok( item ) => {
            match item {
                Some ( _ ) => assert!(false, "FeaturePreference should NOT have been created or updated; id was invalid"), // record should NOT have been created or updated
                None => assert!(true), // record sho uld have been created or updated, if not, fail
            }
        }
        Err (_) => assert!(true), // record should have been created or updated, if not, fail
    };

    // Test 3: Invalid userid 
    let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.upsert_feature_preference(constants::INVALID_OTHER_ID, constants::INVALID_OTHER_ID).await;
    match qry_results{
        Ok( item ) => {
            match item {
                Some ( _ ) => assert!(false, "FeaturePreference should NOT have been created or updated; id was invalid"), // record should NOT have been created or updated
                None => assert!(true), // record sho uld have been created or updated, if not, fail
            }
        }
        Err (_) => assert!(true), // record should have been created or updated, if not, fail
    };
}

/// ### test_ins_get_upd_feature_priority_intv_details_level()
/// 
/// Tests the ability for the DAO to CREATE, SELECT and UPDATE Feature Priority records at the Intervention Details level
/// Specifically tests:
///  * upsert_feature_preference
///  * get_active_feature_preferences_of_intervention_details_for_user
/// 
#[tokio::test]
async fn test_ins_get_upd_feature_priority_intv_details_level() {
  let db_url = DB_CONN_STR;
  let db_pool = match PgPoolOptions::new()
      .max_connections(5)
      .connect(db_url)
      .await
  {
      Ok(pool) => pool,
      Err(e) => {
        panic!("{}", e)
      },
  };

  let mut test_user_id = 2; // we will use user #2, which is admin user and has lots of data (permissions)
  let mut test_upper_limit = 5;
  let mut test_feature_id: i64 = 100002;
  let fp_id: i64;

  println!("Testing: get_all_active_feature_preferences_for_user(): first insertion");

  // Test 1: try to create a FeaturePreference
  // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application

  // Test 2: valid upsert of intervention detail
  let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.upsert_feature_preference(test_user_id, test_feature_id).await;
  match qry_results.unwrap(){
      Some ( results ) => {
          fp_id = results; // record should have been created or updated
          println!("..updated id={}", fp_id);
          assert!(true);
      }
      None => {
        assert!(false); // record should have been created or updated, if not, fail
      }
  }

  let mut new_fp: FeaturePreference = Default::default(); // use a dummy record to satify the compiler below

  println!("..first get: test_user_id={} test_feature_id={} 5 true", test_user_id, test_feature_id);

  // Test 3: retrieve intervention detail to see if it was indeed updated
  let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.get_active_feature_preferences_of_intervention_details_for_user(test_user_id, test_feature_id, test_upper_limit).await;
  match qry_results.unwrap(){
        Some ( results ) => {
            let mut found: bool = false;

            println!("....results.len()={}, looking for ref_group_id={}", results.len(), test_feature_id);

            for item in results{
                println!("....check item.id {}={}", test_feature_id, item.ref_group_id);
                
                if item.ref_group_id == test_feature_id { // ** NOTE: for Intervention-Detail level FPs, we match on the ref_group_id field, not the id as we did at Intervention Level
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

  let updated_fp_id: i64; // = constants::INVALID_OTHER_ID;;

  println!("..update created record: test_user_id={} test_feature_id={} 5 true", test_user_id, test_feature_id);
  // Test 4: try to update the FeaturePreference
  let qry_results = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.upsert_feature_preference(test_user_id, test_feature_id).await;
  match qry_results.unwrap(){
        Some ( results ) => {
            updated_fp_id = results;
            assert!(true); // no magic here, it should just not fail
        }
        None => {
            updated_fp_id = constants::INVALID_OTHER_ID; 
            assert!(false);
        }
  }

  println!("..retrieve updated: test_user_id={} test_feature_id={}, 5 true", test_user_id, test_feature_id); // NOTE: use of constants::CRT_ANY_INTERVENTION_GROUP below, it is a group for intervention-level test ids

  // Test 4b (validation): try to pull out that same that was updated: this time, the updated timestamp should be different than the first time
  let qry_results_updated = get_individual_feature_preference( updated_fp_id).await;
  match qry_results_updated.unwrap(){
      Some ( item ) => {
            let mut updated: bool = false;
            println!("..Some() results, checking record id={} updated", updated_fp_id);
            
            if new_fp.calculation_date != item.calculation_date {
                println!("{} != {}", new_fp.calculation_date.to_string(), item.calculation_date.to_string() );
                updated = true;
            }

            if new_fp.weight != item.weight {
                println!("{} != {}", new_fp.calculation_date.to_string(), item.calculation_date.to_string() );
                updated = true;
            }
        
            assert!( updated );  // if the id was not found, the insert failed
      }
      None => {
            println!("..feature pref id={} not found", updated_fp_id);
            assert!(false);
      }        
    }

    // Test 5: Invalid User Id; should not return rows
    let qry_results2 = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.get_active_feature_preferences_of_intervention_details_for_user(
                                    constants::INVALID_OTHER_ID,
                                    constants::INVALID_OTHER_ID,
                                    test_upper_limit).await;
    match qry_results2.unwrap(){
        Some ( _items ) => {
            assert!(false, "Invalid id (-1) no results expected");
        },
        None => assert!(true), // no results expected
    }

    test_user_id = 2;     // we will use user #2, which is admin user and has lots of data (permissions)
    test_upper_limit = 1; // drop the limit really low for testing
    test_feature_id = 1; // drop the limit really low for testing

    // Test 6: Valid user with lots of Feature Preferences at the Intervention Details level
    // * Low Limit Test
    // * High number of permissions Test    
    let qry_results2 = {FeaturePreferenceDAO::new( db_pool.clone() ).await}.get_active_feature_preferences_of_intervention_details_for_user(
                                    test_user_id,
                                    test_feature_id,
                                    test_upper_limit).await;
    match qry_results2.unwrap(){
        Some ( _items ) => assert!(true),
        None => assert!(false, "Results expected"),
    }

}

    const QRY_GET_INDIVIDUAL_FEATURE_PREFERENCE: &str = r##"
                                                SELECT fp.id, display_order, weight,
                                                    calculation_date,
                                                    COALESCE(users_id, '-1') AS "users_id",
                                                    COALESCE(department_id, '-1') AS "department_id",
                                                    fp.active_flag, feature_id,
                                                    crf.group_id "ref_group_id",
                                                    crf.name "ref_group_name"
                                                FROM feature_preference fp
                                                JOIN common_reference_type crf on fp.feature_id = crf.id
                                                WHERE fp.ID = {feature_preference_id}
                                                         "##; 

    /// ### get_individual_feature_preference()
    /// 
    /// Finds and returns an intervention based on an intervention/id 
    /// This is not needed by the main application code and as such is only present in the unit test
    /// 
    async fn get_individual_feature_preference(feature_preference_id: i64) -> Result< Option< FeaturePreference >, std::io::Error> {
        let query = QRY_GET_INDIVIDUAL_FEATURE_PREFERENCE.replace("{feature_preference_id}", &feature_preference_id.to_string());
        println!("test::get_individual_feature_preference()");

        let db_pool: sqlx::Pool<sqlx::Postgres> = match PgPoolOptions::new()
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

        let rows: Vec<(i64, i32, i32,  // id, display_order, weight, 
                       chrono::NaiveDateTime,  // calculation_date
                       i64, i64,  // users_id, department_id
                       String, i64, // , active_flag, feature_id
                       i64, String // ref_group_id, ref_group_name
                    )> = sqlx::query_as(&query)
        .fetch_all(&db_pool) 
        .await
        .expect("No rows returned");

        println!("..opening connection");
        println!("..rows: {}", rows.iter().count());

        if rows.is_empty() {
            println!("..no feature prefs returned. sql= {}", query);
            return Ok( None );
        }
        else{
            //println!("Loading {} Interventions",  rows.len());
            let mut results: Vec<FeaturePreference> = Vec::with_capacity(rows.len());
            for row in rows { // should only ever iterate once
                let tmp_id: i64 = row.0; // id
                let _tmp_display_order: i32 = row.1; // display_order
                let _tmp_weight: i32 = row.2; // weight

                let tmp_calculation_date: chrono::NaiveDateTime = row.3; // calculation_date
                let tmp_users_id: i64 = row.4; // users_id
                let tmp_department_id: i64  = row.5; // department_id

                let _tmp_active_flag: String = row.6; //active_flag
                let tmp_feature_id: i64 = row.7; //feature_id
                let tmp_ref_group_id: i64 = row.8; // ref_group_id
                let tmp_ref_name: String = row.9; //ref_name

                results.push(
                    FeaturePreference {
                        id: tmp_id,
                        display_order: 0, // we don't care about these for the unit test
                        weight: 0,        //
                        calculation_date: tmp_calculation_date, 
                        users_id: tmp_users_id,
                        department_id: tmp_department_id,
                        feature_id: tmp_feature_id,
                        ref_group_id: tmp_ref_group_id,
                        ref_name: tmp_ref_name
                    }
                );
            }
            return Ok( results.first().cloned() ); // because this is in an enclosure we MUST add the return keyword for it to compile
        }
    }

    /// ### delete_preference()
    /// 
    /// Deletes the preferences for User_id, which has been designated for these unit tests
    /// 
    async fn delete_preference(){
        const SQL_DELETE_PREFERENCES_FOR_USERID_7: &str = r##"delete from feature_preference where users_id = 7"##;     
        let query = SQL_DELETE_PREFERENCES_FOR_USERID_7.to_string();

        let db_pool: sqlx::Pool<sqlx::Postgres> = match PgPoolOptions::new()
            .max_connections(5)
            .connect(DB_CONN_STR)
            .await   {
            Ok(pool) => pool,
            Err(e) => panic!("{}", e),
        };

        let result = sqlx::query(&query)
                                                        .fetch_optional(&db_pool)
                                                        .await
                                                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
        match result {
            Ok( _item ) => println!("Preferences for user_id=7 deleted"),
            Err(e) => println!("Error on delete preferences for user_id=7: {}", e)
        }
    }