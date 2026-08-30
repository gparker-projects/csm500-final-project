//! -------------------------------------------------------------------
//! Unit & Integration tests for Interventions Routes
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//!   Unit Testing in Rust is actually easy! - Flo Woelki (https://youtu.be/6wAFdBVJbwc?si=KdJfqvRdcXi9-mqo) - LOL NOT easy
//!   https://doc.rust-lang.org/book/ch11-03-test-organization.html
//! 
//! -------------------------------------------------------------------
//use chrono::{Utc, NaiveDateTime};
//use rand::{RngExt, rng};
use sqlx::postgres::{PgPoolOptions}; 
use std::env;
//use actix_session::{storage::CookieSessionStore, SessionMiddleware}; //, storage::RedisSessionStore}
use std::sync::Arc;
use ort::{	session::{Session, builder::GraphOptimizationLevel} };
//use actix_web::cookie::Key;

//use common::test_utils::*; 
use maple_emr::{constants, dto::{encounter::*, intervention::*, intervention_detail::*, patient::*, user::*}};
use maple_emr::dto::user_auth::*;

use maple_emr::route::intervention_details_route::InterventionDetailsRoute;
use maple_emr::ui::data_forms::InterventionDetailsDataForm;
use maple_emr::session::{AppSession, UserSession};

use maple_emr::ui::tile_factory::WebContentFactory; 

pub const DB_CONN_STR : &str = "postgres://postgres:csm500@localhost:5432/csm500";

mod common;

#[cfg(test)]

#[tokio::test]

async fn test_route_to_intervention_detail_save() {
    //async fn route_to_intervention_detail_save(app_session: web::Data<AppSession>, user_session: Session, mut req: web::Form<InterventionDetailsDataForm>) -> impl Responder {

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

    let base_model_dir = match env::var(constants::CARGO_MANIFEST_DIR) {
        Ok(tmp_path) => {
            tracing::info!("CARGO_MANIFEST_DIR = {}", tmp_path);
            tmp_path
        }
        Err(e) => {
            tracing::error!("CARGO_MANIFEST_DIR not set: {}", e);
            "INVALID_PATH".to_string()
        }
    } + constants::DATA_SUB_DIRECTORY;
    
    let nle_session: ort::session::Session = Session::builder().expect("Session could not be established")
                  .with_optimization_level(GraphOptimizationLevel::Level1).expect("No Session")
                  .with_intra_threads(1).expect("Insufficient threads")
                  .commit_from_file(&(base_model_dir.clone() + &"all-MiniLM-L6-v2.onnx".to_string()) ).expect("File could not be accessed"); 
    let shared_session = Arc::new(nle_session);

    let app_key = 
    actix_web::cookie::Key::from(
    std::env::var("SESSION_KEY")
        .unwrap_or_else(|_| "this_is_a_new_system_key_to_prevent_regeneration_of_a_key_every_time_the_app_starts".to_string())
        .as_bytes()
    );

    let cfg: SysConfig;

    let app_session = AppSession {
        wcf: WebContentFactory::new(&base_model_dir),
        app_key: app_key,
        connection: db_pool,
        system_config: cfg,
        nle_session: Arc::clone(&shared_session), 
    };


    let perm1= Permission {
        department_id: 4,
        permission_id: 1
    };

    let perm2= Permission {
        department_id: 4,
        permission_id: 2
    };

    let mut auths = Vec::<Permission>::new();
    auths.push(perm1);
    auths.push(perm2);
    
    let user_session = UserSession {
        user_id: "TEST_USER".to_string(),
        user_display_name: "USER, UNIT TEST".to_string(),
        email: "TEST@TESTUSER.COM".to_string(),
        user_authorizations: UserAuthorization { 
            granted_permissions: auths,
        },  
    };

    let frm: InterventionDetailsDataForm = InterventionDetailsDataForm {
        intervention_details_id: constants::INVALID_OTHER_ID.to_string(),
        intervention_id: constants::INVALID_OTHER_ID.to_string(),
        type_id: 30.to_string(), 
        value: 200.to_string(),
        notes: "Very short".to_string(),
        entry_timestamp: "2026-08-02 15:00:00".to_string(),
        form_errors: "".to_string(),
    };

    //let results = InterventionDetailsRoute::route_to_intervention_detail_save(web::Data<&app_session>, user_session, frm);

    assert!(true);
    //assert_eq!(body_str, r#"{"message":"Hello world!"}"#);
}
