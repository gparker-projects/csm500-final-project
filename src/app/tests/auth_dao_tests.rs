/// #Unit & Integration tests for the AuthDAO module
/// 
/// * new()
/// * can_user_login()
/// * get_user_and_departments_at_current_user_sites()
///
/// #### Refs
/// * Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy

mod common;

use sqlx::postgres::PgPoolOptions; 
use tracing;
use maple_emr::{constants, dao::auth_dao::AuthDAO};
use chrono::NaiveDateTime;

//use maple_emr::constants;

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[cfg(test)]

/// ### test_can_user_login()
/// 
/// Tests the ability for the DAO to get data to confirm the user can log into the system
/// 
///   Specifically tests: AuthDAO::can_user_login() 
///
#[tokio::test]
async fn test_can_user_login() {
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

    let test_user_name = "mma2".to_string();
    let test_password = "csm500".to_string();
    let expected_date_time: NaiveDateTime = NaiveDateTime::parse_from_str(&"2026-08-04 13:23:41".to_string(), &"%Y-%m-%d %H:%M:%S".to_string()).unwrap();

    // instantiate a DAO to prove it can access data, but more importantly, detect unexpected changes to it that will break the application
    // we can not test if the DAO itself is instantiated as the only content is a PgPool, which does not allow assert_eq!. If the object
    // does not instantiate however, the remainder of this test will fail.
    let adao = AuthDAO::new( db_pool.clone() );
    
    // Test 1: the valid username/pw combo
    let results = adao.await.clone().can_user_login( test_user_name.clone(), test_password.clone() ).await.unwrap();
    match results {
        Some( obj ) => {
            assert!( obj.user_name == test_user_name.clone(), "Different user returned: {}", obj.user_name.clone());
            assert!( obj.password == test_password.clone(), "Different password returned: {}", obj.password.clone());
            assert!( obj.email == "mma2@google.com", "Different email returned: {}", obj.email.clone());
            assert!( obj.name == "Mattie Medical AssistantTwo", "Different name returned: {}", obj.name.clone());
            assert!( obj.created_timestamp == expected_date_time, "Different created_timestamp returned: {}", obj.created_timestamp.clone());
        },
        None => assert!( false, "User not returned as expected" ),
    };

    // Test 2: invalid pw, valid username combo
    let results = {AuthDAO::new( db_pool.clone() )}.await.can_user_login( test_user_name.clone(), "bad_passowrd".to_string() ).await.unwrap();
    match results {
        Some( _obj ) => assert!( false, "User was incorrectly returned, which should not have occurred" ),
        None => assert!( true, "User was NOT returned, as expected" ),
    };

    // Test 3: invalid username, "valid" pw (used by some other account) combo
    let results = {AuthDAO::new( db_pool.clone() )}.await.can_user_login( "mma3".to_string() , test_password.clone()).await.unwrap();
    match results {
        Some( _obj ) => assert!( false, "User was incorrectly returned, which should not have occurred" ),
        None => assert!( true, "User was NOT returned, as expected" ),
    };

    // Test 4: invalid username, invalid combo - username exceeds column length for username (50 chars)
    let results = {AuthDAO::new( db_pool.clone() )}.await.can_user_login( "01234567890123456789012345678901234567890123456789xx".to_string() , test_password.clone()).await.unwrap();
    match results {
        Some( _obj ) => assert!( false, "User was incorrectly returned, which should not have occurred" ),
        None => assert!( true, "User was NOT returned, as expected" ),
    };

    // Test 5: invalid username, invalid combo - pw exceeds column length for username (50 chars)
    let results = {AuthDAO::new( db_pool.clone() )}.await.can_user_login( test_user_name.clone(), "01234567890123456789012345678901234567890123456789xx".to_string() ).await.unwrap();
    match results {
        Some( _obj ) => assert!( false, "User was incorrectly returned, which should not have occurred" ),
        None => assert!( true, "User was NOT returned, as expected" ),
    };

    // Test 6: invalid username, invalid combo - pw exceeds column length for username (50 chars)
    let results = {AuthDAO::new( db_pool.clone() )}.await.can_user_login( "; SELECT * FROM INVALID;".to_string(), "; SELECT * FROM INVALID;".to_string() ).await.unwrap();
    match results {
        Some( _obj ) => assert!( false, "(Error Expected) User was incorrectly returned, which should not have occurred" ),
        None => assert!( true, "(Error Expected) User was NOT returned, as expected" ),
    };
}


/// ### test_get_user_permissions()
/// 
/// Tests the ability for the DAO to obtain user permissions based on a userid
/// 
///   Specifically tests: AuthDAO::get_user_permissions() 
///
#[tokio::test]
async fn test_get_user_permissions() {
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
    let test_user_id: i64 = 4; // Mattie Medical AssistantTwo
    let adao = AuthDAO::new( db_pool.clone() );
    
    // Test 1: valid userid
    let results = adao.await.clone().get_user_permissions( test_user_id ).await.unwrap();
    match results {
        Some( obj ) => {
            assert!( obj.granted_permissions.len() == 8, "More/less permissions {} than expected (8)", obj.granted_permissions.len());
        },
        None => assert!( false, "UserAuthorization (permission set) not returned as expected" ),
    };

    // Test 2: invalid userid
    let results = {AuthDAO::new( db_pool.clone() )}.await.clone().get_user_permissions( constants::INVALID_OTHER_ID ).await;
    match results {
        Ok( item) => {
            match item {
                Some( _obj ) => {
                    assert!( false, "UserAuthorization (permission set) was returned, when not expected" )
                },
                None => assert!( true, "No permissions returned, as expected" ),
            };
        },
        Err( _ ) => assert!( true, "No permissions returned, as expected" ),
    }
}

/// ### test_get_user_and_departments_at_current_user_sites()
/// 
/// Tests the ability for the DAO to obtain the usernames and their departments at the users' site
/// 
///   Specifically tests: AuthDAO::get_user_and_departments_at_current_user_sites() 
///
#[tokio::test]
async fn test_get_user_and_departments_at_current_user_sites() {
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
    let test_user_id: i64 = 4; // Mattie Medical AssistantTwo
    
    // Test 1: valid userid, exact number of expected rows
    let results = {AuthDAO::new( db_pool.clone() )}.await.clone().get_user_and_departments_at_current_user_sites( test_user_id ).await.unwrap();
    match results {
        Some( items ) => assert!( items.len() == 10, "Expected 10 Departments, retrieved {} ", items.len()),
        None => assert!( false, "No Departments returned; expected 10" ),
    };

    // Test 2: invalid userid
    let results = {AuthDAO::new( db_pool.clone() )}.await.clone().get_user_and_departments_at_current_user_sites( constants::INVALID_OTHER_ID ).await.unwrap();
    match results {
        Some( items ) => {
            if items.len() != 0{ 
                assert!( false, "Departments were returned, when none were expected" )
            }
            else {
                assert!( true, "No Departments were returned, as expected")
            }
        },
        None => assert!( true, "No Departments were returned, as expected"),
    };
}