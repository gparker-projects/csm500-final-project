/// -------------------------------------------------------------------
/// #Unit & Integration tests for the Session, SysConfig and Convert Utils module
/// 
/// ###Includes:
///
/// Ref: Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
///  CSM500 Project (April - October 2026)
///  Graham Parker (Student ID: 240120522)
/// -------------------------------------------------------------------

mod common;

#[cfg(test)]

use maple_hms::constants;
use maple_hms::route::patient_route::*;
use maple_hms::session::*;
use maple_hms::ui::data_forms::*;
use maple_hms::dto::user_auth::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use common::entity_factory::EntityFactory;

const SCREEN_ID_TAG_PATIENT_LIST : &str = "<div id=\"MapleHMS::ID=PatientListTile\"></div>";

/// ### test_patient_route_route_to_patient_details()
/// 
/// Tests:
///   PatientRoute::route_to_patient_details()
/// 
#[actix_web::test]
async fn test_patient_route_route_to_patient_details(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let mut frm = GenericWebFormData { target_id: "1".to_string() };

    // Test 1: HomeRoute::route_to_home, standard call with a clinical user
    println!("Test 1: PatientRoute::route_to_patient_details with clinical data (only) user");
    let responder = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( SCREEN_ID_TAG_PATIENT_LIST ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };

    // Test 2: PatientRoute::route_to_patient_details, standard call with a non-clinical + clinical permissioned user 
    println!("Test 2: PatientRoute::route_to_patient_details: non-clinical data + clinical data permissioned user");
    let mut tmp_us = EntityFactory::get_mock_user_session("2".to_string());   // base 
    tmp_us.user_authorizations.granted_permissions.push( Permission::new(1, Permission::ALLOW_CREATE_NON_CLINICAL_INTERVENTION) );
    let _ignore2 = user_session.insert(constants::USER_SESSION, tmp_us); // we will ignore error as this is a unit test

    let responder2 = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp2 = responder2.respond_to(&req);

    assert_eq!(http_resp2.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp2.into_body()).await{
        Ok(item) => {
            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( SCREEN_ID_TAG_PATIENT_LIST ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };

    // Test 3: PatientRoute::route_to_patient_details, Patient with no Encounters
    println!("Test 3: PatientRoute::route_to_patient_details: Patient with no Encounters");

    frm.target_id = "31".to_string(); // known patient 31 does not have any Encounters
    let responder_test3 = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp_test3 = responder_test3.respond_to(&req);

    assert_eq!(http_resp_test3.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp_test3.into_body()).await{
        Ok(item) => {
            //println!("Body = {}", String::from_utf8_lossy(&item));

            // confirm the content of the screen was loaded correctly by: locating fake PHN for target patient
            const SEARCH_TEXT : &str = "name=\"patient_id\" value=\"31\">";
            assert!( String::from_utf8_lossy(&item).contains( SEARCH_TEXT ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };

    // Test 4: PatientRoute::route_to_patient_details, Patient with no Encounters
    println!("Test 4: PatientRoute::route_to_patient_details: Patient with many interventions");

    frm.target_id = "1".to_string(); // known patient 1 has > 99 encounters in test data set
    let responder_test3 = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp_test3 = responder_test3.respond_to(&req);

    assert_eq!(http_resp_test3.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp_test3.into_body()).await{
        Ok(item) => {
            //println!("Body = {}", String::from_utf8_lossy(&item));

            // confirm the content of the screen was loaded correctly by: locating fake PHN for target patient
            const SEARCH_TEXT : &str = "name=\"patient_id\" value=\"1\">";
            assert!( String::from_utf8_lossy(&item).contains( SEARCH_TEXT ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    };

    // Test 6: PatientRoute::route_to_patient_details, Patient without interventions
    println!("Test 6: PatientRoute::route_to_patient_details: Patient without interventions");
    frm.target_id = "9".to_string(); // known patient 1 has > 99 encounters in test data set
    let responder_test3 = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp_test3 = responder_test3.respond_to(&req);

    assert_eq!(http_resp_test3.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp_test3.into_body()).await{
        Ok(item) => {
            //println!("Target Body = {}", String::from_utf8_lossy(&item));

            // confirm the content of the screen was loaded correctly by: locating fake PHN for target patient
            const SEARCH_TEXT : &str = "No Interventions found";
            assert!( String::from_utf8_lossy(&item).contains( SEARCH_TEXT ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    }

    // Test 7: PatientRoute::route_to_patient_details, User is non-clinical, limiting permissions
    println!("Test 7: PatientRoute::route_to_patient_details: User is non-clinical");
    let perm_test_7: Permission = Permission::new(1,  Permission::ALLOW_CREATE_UPDATE_ADMIT);   // create some base permissions
    let perms_test_7: Vec<Permission> = vec![perm_test_7]; // put them in a vector
    let tmp_us_test_7 = UserSession { // the UserAuthorization gets  put into the UserSession
        user_id: "3".to_string(), 
        user_display_name: "TEST, UNIT".to_string(), 
        email: "test@gmail.com".to_string(), 
        user_authorizations: UserAuthorization { granted_permissions: perms_test_7 } 
    };
    let _ignore_test7 = user_session.insert(constants::USER_SESSION, tmp_us_test_7); // we will ignore error as this is a unit test
    let responder_test7 = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp_test7 = responder_test7.respond_to(&req);

    assert_eq!(http_resp_test7.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp_test7.into_body()).await{
        Ok(item) => {
            //println!("Target Body = {}", String::from_utf8_lossy(&item));

            // confirm the content of the screen was loaded correctly by: locating fake PHN for target patient
            const SEARCH_TEXT : &str = "value='100041' >Appointments</option>"; //
            assert!( String::from_utf8_lossy(&item).contains( SEARCH_TEXT ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    }

    // Test 8: PatientRoute::route_to_patient_details, User does not have any patients at their site (#2)
    println!("Test 8: PatientRoute::route_to_patient_details: User 9 does not have any patients at their site (#2)");
    let perm_test_8: Permission = Permission::new(1,  Permission::ALLOW_CREATE_UPDATE_ADMIT);   // create some base permissions
    let perms_test_8: Vec<Permission> = vec![perm_test_8]; // put them in a vector
    let tmp_us_test_8 = UserSession { // the UserAuthorization gets  put into the UserSession
        user_id: "9".to_string(), 
        user_display_name: "TEST, UNIT".to_string(), 
        email: "test@gmail.com".to_string(), 
        user_authorizations: UserAuthorization { granted_permissions: perms_test_8 } 
    };
    let _ignore_test8 = user_session.insert(constants::USER_SESSION, tmp_us_test_8); // we will ignore error as this is a unit test
    let responder_test8 = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp_test8 = responder_test8.respond_to(&req);

    assert_eq!(http_resp_test8.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp_test8.into_body()).await{
        Ok(item) => {
            //println!("Target Body = {}", String::from_utf8_lossy(&item));

            // confirm the content of the screen was loaded correctly by: locating fake PHN for target patient
            const SEARCH_TEXT : &str = "patient_id\" value=\"9\""; //
            assert!( String::from_utf8_lossy(&item).contains( SEARCH_TEXT ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    }
}

/// ### test_patient_route_route_to_patient_details()
/// 
/// Tests:
///   PatientRoute::route_to_patient_details()
/// 
#[actix_web::test]
async fn test_patient_route_route_to_patient_details_pt2(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

    let frm = GenericWebFormData { target_id: "302".to_string() }; // patient_id

    // Test 1: PatientRoute::route_to_patient_details, Patient has many interventions
    println!("Test 1: PatientRoute::route_to_patient_details: Patient has many interventions");
    
    let tmp_us_test_1 = UserSession { // the UserAuthorization gets  put into the UserSession
        user_id: "3".to_string(), 
        user_display_name: "TEST, UNIT".to_string(), 
        email: "test@gmail.com".to_string(), 
        user_authorizations: UserAuthorization { granted_permissions: vec![ Permission::new(2,  1),
                                                                            Permission::new(2,  2) ] } 
    };
    let _ignore_test_1 = user_session.insert(constants::USER_SESSION, tmp_us_test_1); // we will ignore error as this is a unit test
    let responder_test_1 = PatientRoute::route_to_patient_details( user_session.clone(), app_session.clone(), web::Form( frm.clone() ) ).await;
    let http_resp_test_1 = responder_test_1.respond_to(&req);

    assert_eq!(http_resp_test_1.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp_test_1.into_body()).await{
        Ok(item) => {
            //println!("Target Body = {}", String::from_utf8_lossy(&item));

            // confirm the content of the screen was loaded correctly by: locating fake PHN for target patient
            const SEARCH_TEXT : &str = "patient_id\" value=\"302\""; //
            assert!( String::from_utf8_lossy(&item).contains( SEARCH_TEXT ), "Response did not contain expected content"); 
        },
        Err(_e) => assert!( false, "Error Response received" ),
    }
}