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

    // Test 3&4 are for the database pool
    let pool = MapleHMSKernel::init_database_pool( "postgres://postgres:csm500@localhost:5432/csm500".to_string() ).await;
    assert!(Some( pool.clone() ).is_some(), "Test 3: Database pool not created");
    assert!(! pool.is_closed(), "Test 4: Database pool not open");

    // Test 5 is the path
    let static_path = MapleHMSKernel::get_static_base_path();
    assert_ne!(static_path, String::new(), "Test 5: Static base path not initialized");
}