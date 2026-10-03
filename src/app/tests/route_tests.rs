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
use maple_hms::ui::tile_factory::WebContentFactory;
use maple_hms::route::default_route::*;
use maple_hms::route::login_route::*;
use maple_hms::route::home_route::*;
use maple_hms::route::patient_route::*;

use maple_hms::session::*;
use maple_hms::ui::data_forms::*;
use maple_hms::dto::user_auth::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use actix_web::cookie::Key;
use sqlx::postgres::PgPoolOptions;

const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";
const SCREEN_ID_TAG_LOGIN : &str = "<div id=\"MapleHMS::ID=Login\"></div>";
const SCREEN_ID_TAG_HOME : &str = "<div id=\"MapleHMS::ID=Home\"></div>";
const SCREEN_ID_TAG_INTERVENTION : &str = "<div id=\"MapleHMS::ID=Intervention\"></div>";
const SCREEN_ID_TAG_PATIENT_LIST : &str = "<div id=\"MapleHMS::ID=PatientListTile\"></div>";



/// ### test_basic_route_default_route()
/// 
/// Tests:
///  BasicRoute::default_route()
///  BasicRoute::is_it_up_route()
///  BasicRoute::not_found_route()
///
 #[actix_web::test]
async fn test_basic_route_default_route(){
    let req = test::TestRequest::default().to_http_request();
    let base_session = req.get_session();
    let app_session = get_mock_app_session().await;

    // Test 1: BasicRoute::default_route
    let responder = BasicRoute::default_route(app_session.clone(), base_session.clone()).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            //let body = String::from_utf8_lossy(&item);
            //println!("Body = {}", body);

            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains(SCREEN_ID_TAG_LOGIN ), "Response did not contain expected content"); 
        },
        Err(_e) => { 
            assert!(false, "Error Response received");
        },
    };

    // Test 2: BasicRoute::default_route with VALIDATION_ERRORS
    let _ignore = base_session.insert(constants::VALIDATION_ERRORS, "Unit Test Error");
    let responder2 = BasicRoute::default_route(app_session.clone(), base_session).await;
    let http_resp2 = responder2.respond_to(&req);
    assert_eq!(http_resp2.status(), StatusCode::OK, "Status not OK");
    match to_bytes(http_resp2.into_body()).await{
        Ok(item) => {
            //let body = String::from_utf8_lossy(&item);
            //println!("Body = {}", body);

            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( SCREEN_ID_TAG_LOGIN ), "Response did not contain expected content"); 
        },
        Err(_e) => { 
            assert!(false, "Error Response received");
        },
    };


    // Test 3: BasicRoute::is_it_up_route
    let responder = BasicRoute::is_it_up_route().await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");

    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            //let body = String::from_utf8_lossy(&item);
            //println!("Body = {}", body);
            const SEARCH_TEXT : &str = "MapleHMS is Up";

            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( SEARCH_TEXT ), "Response did not contain expected content"); 
        },
        Err(_e) => { 
            assert!(false, "Error Response received");
        },
    };

    // Test 4: BasicRoute::is_it_up_route
    let responder = BasicRoute::not_found_route().await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::SEE_OTHER, "Page was not redirected as expected");
}

/// ### get_mock_user_session() 
/// 
/// Supporting method that sets up an initial mock user session for use by the main tests
/// 
fn get_mock_user_session(user_id: String) -> UserSession{
    let perm: Permission = Permission::new(1, 1);   // create some base permissions
    let perm2: Permission = Permission::new(1, Permission::ALLOW_CREATE_CLINICAL_INTERVENTION);
    let perms: Vec<Permission> = vec![perm, perm2]; // put them in a vector
    let ua = UserAuthorization { // the vector gets put into the UserAuthorization
        granted_permissions: perms
    };

    UserSession { // the UserAuthorization gets  put into the UserSession
        user_id: user_id, 
        user_display_name: "TEST, UNIT".to_string(), 
        email: "test@gmail.com".to_string(), 
        user_authorizations: ua 
    }
}

/// ### get_mock_app_session() 
/// 
/// Supporting method that sets up an initial mock application session for use by the main tests
/// 
async fn get_mock_app_session() -> web::Data<maple_hms::session::AppSession> {
   // a lot of set up to mimic a live system session
    let path = std::env::current_dir().expect("Base path to executable could not be found");
    let newpath = path.display().to_string() + "\\webc\\static\\";
    let tmp_wcf = WebContentFactory::new(&newpath, "UNIT TEST".to_string());

    // set up PgPool
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

    let tmp_cargo_manifest_dir = "\\data\\manifest_dir".to_string();
    let tmp_command_mapping_file = "command_mapping.csv".to_string();
    let tmp_language_model_file = "all-MiniLM-L6-v2.onnx".to_string();
    let tmp_tokenizer_file = "tokenizer.json".to_string();
    let tmp_model_data_dir = "\\data\\".to_string();
    let tmp_data_sub_dir = "\\data\\".to_string();

    // set up SysConfig
    let cfg = SysConfig{
        app_version: "v1.0Unit_test".to_string(),
        db_conn_str: DB_CONN_STR.to_string(),
        cargo_manifest_dir: tmp_cargo_manifest_dir, 
        model_data_dir: tmp_model_data_dir,
        command_mapping_file: tmp_command_mapping_file,
        language_model_file: tmp_language_model_file, 
        tokenizer_file: tmp_tokenizer_file, 
        data_sub_dir: tmp_data_sub_dir, 
        max_general_fastactions: "3".to_string(),
        max_nle_fastactions: "3".to_string(),
        website_bind_address: "10.10.10.10:8080".to_string(),
        session_key: "thisIsAVeryinauthenticSessionKeyOnlyToBeused_forunit_testing".to_string(),
        max_age_feature_preferences: "30".to_string()
    };

    // finally: return the initialized AppSession
    web::Data::new(AppSession {
        wcf: tmp_wcf, 
        app_key: Key::generate(),
        connection: db_pool,
        system_config: cfg
    })
}

/// ### test_login_logout_route()
/// 
/// Tests:
///  LoginRoute::login()
///  LoginRoute::logout()
///
#[actix_web::test]
async fn test_login_logout_route(){
    let req = test::TestRequest::default().to_http_request();
    let user_session = req.get_session();
    let app_session = get_mock_app_session().await;

    let mut frm = LoginFormData {
                username: "mma2".to_string(),
                password: "csm500".to_string(),
    };

    // Test 1: LoginRoute::login with valid userid
    println!("Test 1: LoginRoute::login with valid userid");
    //login(user_session: Session, req: web::Form<LoginFormData>, app_session: web::Data<session::AppSession> )
    let responder = LoginRoute::login(user_session.clone(), web::Form( frm.clone() ), app_session.clone()).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::SEE_OTHER, "Status not SEE_OTHER");

    // Test 2: LoginRoute::login with invalid userid
    println!("Test 2: LoginRoute::login with invalid userid");
    frm.username = "UNIT_TEST_INVALID_USERID".to_string();
    //login(user_session: Session, req: web::Form<LoginFormData>, app_session: web::Data<session::AppSession> )
    let responder = LoginRoute::login(user_session.clone(), web::Form( frm ), app_session.clone()).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::SEE_OTHER, "Status not SEE_OTHER");

    // Test 3: LoginRoute::logout
    //logout(user_session: Session )
    println!("Test 3: LoginRoute::logout");
    let responder = LoginRoute::logout( user_session.clone() ).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::SEE_OTHER, "Status not SEE_OTHER");
}

/// ### test_home_route_route_to_home()
/// 
/// Tests:
///   HomeRoute::route_to_home()
/// 
#[actix_web::test]
async fn test_home_route_route_to_home(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = get_mock_app_session().await; // create app session

    // Test 1: HomeRoute::route_to_home, standard call with a regular user
    println!("Test 1: HomeRoute::route_to_home");
  
    let responder = HomeRoute::route_to_home( app_session.clone(), user_session.clone()).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");

    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( SCREEN_ID_TAG_HOME ), "Response did not contain expected content"); 
        },
        Err(_e) => {
            assert!( false, "Error Response received" );
        },
    };

    // Test 2: HomeRoute::route_to_home, standard call with a user who does not return a patient list. Easy: use an invalid user id
    println!("Test 2: HomeRoute::route_to_home, user does not have a patient list");
    let _ignore2 = user_session.insert(constants::USER_SESSION, get_mock_user_session("-1".to_string())); // we will ignore error as this is a unit test

    let responder = HomeRoute::route_to_home( app_session.clone(), user_session.clone()).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");

    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            // confirm the content of the screen was loaded correctly by detecting a tag only present in the key Tile template file
            assert!( String::from_utf8_lossy(&item).contains( SCREEN_ID_TAG_HOME ), "Response did not contain expected content"); 
        },
        Err(_e) => {
            assert!( false, "Error Response received" );
        },
    };
    //assert!( false, "DEBUG" );
}


/// ### test_patient_route_route_to_patient_details()
/// 
/// Tests:
///   PatientRoute::route_to_patient_details()
/// 
#[actix_web::test]
async fn test_patient_route_route_to_patient_details(){
    let req = test::TestRequest::default().to_http_request(); // create a user request
    let user_session = req.get_session();  // create user_session
    let _ignore = user_session.insert(constants::USER_SESSION, get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = get_mock_app_session().await; // create app session

    let frm = GenericWebFormData { target_id: "1".to_string() };

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

    // Test 2: HomeRoute::route_to_home, standard call with a non-clinical + clinical permissioned user 
    println!("Test 2: PatientRoute::route_to_patient_details: non-clinical data + clinical data permissioned user");
    let mut tmp_us = get_mock_user_session("2".to_string());   // base 
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
}


/// ### test_nle_route_natural_language_prompt()
/// 
/// Tests:
///   NLERoute::natural_language_prompt()
/// 
#[actix_web::test]
async fn test_nle_route_natural_language_prompt(){

    //Test 1: context level 1

    //Test 2: context level 2

    //Test 3: context level 0

    //Test 4a: OTHER_PATIENT_FOUND 

    //Test 4b: TARGET_PATIENT_FOUND  

    //Test 4c: all other cases 
}