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

use maple_hms::route::default_route::*;
use maple_hms::route::login_route::*;
use maple_hms::route::home_route::*;
use maple_hms::ui::data_forms::*;
use common::entity_factory::EntityFactory;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};


const SCREEN_ID_TAG_LOGIN : &str = "<div id=\"MapleHMS::ID=Login\"></div>";
const SCREEN_ID_TAG_HOME : &str = "<div id=\"MapleHMS::ID=Home\"></div>";

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
    let app_session = EntityFactory::get_mock_app_session().await;

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

    // Test 5: BasicRoute::error_route
    let responder = BasicRoute::error_route().await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::SERVICE_UNAVAILABLE, "MapleHMS is currently down; no redirection");
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
    let app_session = EntityFactory::get_mock_app_session().await;

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
    let _ignore = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("1".to_string())); // we will ignore error as this is a unit test
    let app_session = EntityFactory::get_mock_app_session().await; // create app session

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
    let _ignore2 = user_session.insert(constants::USER_SESSION, EntityFactory::get_mock_user_session("-1".to_string())); // we will ignore error as this is a unit test

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
