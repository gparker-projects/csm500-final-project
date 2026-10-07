/// -------------------------------------------------------------------
/// # Tests kernal methods that have been extracted from the main.rs
/// 
/// ### Includes:
/// * MapleHMSKernel::get_static_base_path()
/// * MapleHMSKernel::init_database_pool()
/// * MapleHMSKernel::init_logging()
/// 
/// CSM500 Project (April - October 2026)
/// Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

#[cfg(test)]
use std::fs;

use maple_hms::kernel::MapleHMSKernel;

/// ### test_kernal_methods() 
/// 
/// Tests:
///  MapleHMSKernel::get_static_base_path()
///  MapleHMSKernel::init_database_pool()
///  MapleHMSKernel::init_logging()
///
 #[actix_web::test]
async fn test_kernal_methods(){
    // Test 1 & 2 are for logging
    let logging_file = MapleHMSKernel::init_logging();
    assert_ne!(logging_file, String::new(), "Test 1: Logging file name invalid");
    assert!(fs::exists(logging_file).unwrap_or(false), "Test 2: Logging file was not created"); // confirms file was actually created

    // Test 3a and b are for the database pool - valid
    let pool: Option<sqlx::Pool<sqlx::Postgres>> = MapleHMSKernel::init_database_pool( "postgres://postgres:csm500@localhost:5432/csm500".to_string() ).await;
    assert!( pool.is_some() , "Test 3a: Database pool not created");
    assert!(! pool.clone().unwrap().is_closed(), "Test 3b: Database pool not open");

    // Test 4a are for the database pool - invalid; if there is no pool, we can not unwrap, so only one test
    let pool = MapleHMSKernel::init_database_pool( "postgres://postgres:fake@localhost:666/csm500".to_string() ).await;
    assert!( pool.is_none() , "Test 4a: Database pool created when it should not have been");

    // Test 5 is the path
    let static_path = MapleHMSKernel::get_static_base_path();
    assert_ne!(static_path, String::new(), "Test 5: Static base path not initialized");
    println!("Test 5 completed");

    // Test 6: run the main()
    //let value = MapleHMSKernel::main().await.;
    //assert!( value.is_ok(), "Test 6: Main() did not initialize");
    //println!("Test 6 completed");
}