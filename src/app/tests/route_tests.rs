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

//use maple_hms::dto::user_auth::*;

use maple_hms::ui::tile_factory::WebContentFactory;
//use maple_hms::session::UserSession; // AppSession
use maple_hms::route::default_route::*;
use maple_hms::session::*;

use actix_session::SessionExt;
use actix_web::{body::to_bytes, http::StatusCode, test, web, Responder};
use actix_web::cookie::Key;
use sqlx::postgres::PgPoolOptions;

const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

const LOGIN_SCREEN_ID_TAG : &str = "<div id=\"MapleHMS::ID=Login\"></div>";

/// ### test_default_route()
/// 
/// Tests DefaultRoute::default_route()
//#[tokio::test]
 #[actix_web::test]
async fn test_default_route(){

    let req = test::TestRequest::default().to_http_request();
    let base_session = req.get_session();
    let app_session = get_mock_app_session().await;

    let responder = BasicRoute::default_route(app_session, base_session).await;
    let http_resp = responder.respond_to(&req);
    assert_eq!(http_resp.status(), StatusCode::OK, "Status not OK");

    match to_bytes(http_resp.into_body()).await{
        Ok(item) => {
            let body = String::from_utf8_lossy(&item);
            println!("Body = {}", body);

            assert!( String::from_utf8_lossy(&item).contains( LOGIN_SCREEN_ID_TAG ), "Response did not contain expected content"); 
        },
        Err(_e) => { 
            assert!(false, "Error Response received");
        },
    };
}

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

/*
async fn get_mock_user_session() -> UserSession {
  // create a couple of permission and put them in a Vector, then add to the UserAuthorization
    // the UserAuthorization gets put into the Session
    let perm: Permission = Permission::new(1, 1);
    let perm2: Permission = Permission::new(1, 2);

    let perms: Vec<Permission> = vec![perm, perm2];

    let ua = UserAuthorization {
        granted_permissions: perms
    };

    UserSession {
        user_id: "1".to_string(), // 
        user_display_name: "TEST, UNIT".to_string(), //
        email: "test@gmail.com".to_string(), //
        user_authorizations: ua //
    }
}

References:
 https://bitskingdom.com/blog/web-apps-rust-performance-optimization/


#[actix_web::test]
use reqwest;
use tokio;

#[cfg(test)] 

/// ### test_create_encounter_dto()
/// 
/// Tests the ability to create an Encounter DTO and its basic methods:
/// * admit_timestamp_for_display()
/// * to_string() - trait override
/// 
#[test]
async fn test_reqwest() {
    let response = reqwest::get("/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("Response: {}", response);
}*/