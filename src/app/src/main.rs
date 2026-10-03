//! ---------------------------------------------------------------------------------
//! Main program executable for the project
//!
//! CSM500 Project (April - October 2026)
//! Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! Refs for Web and DB:
//! [1] B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
//!   https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
//!   https://github.com/Rust-Web-Development/code
//!   user session management: https://docs.rs/actix-session/latest/actix_session/
//!   config file: Practical Rust Projects, pg 337-339; https://mojoauth.com/parse-and-generate-formats/parse-and-generate-toml-with-rust#error-handling-and-validation-with-toml
//! 
//! Refs for ML code:
//! [2] S. Lyu and A. Rzeznik, Practical Rust Projects: Build Serverless, AI, Machine Learning, Embedded, Game, and Web Applications. Berkeley, CA: Apress, 2023. doi: DOI:%2010.1007/978-1-4842-9331-7.
//!   https://github.com/LukeMathWalker/zero-to-production
//! 
//! ---------------------------------------------------------------------------------

use actix_cors::Cors;
use actix_files::*;
use actix_session::{storage::CookieSessionStore, SessionMiddleware}; //, storage::RedisSessionStore}
use actix_web::{web, App, HttpServer};
use std::env;

use sqlx::postgres::{PgPoolOptions};
use tracing;
use tracing_subscriber::{ Layer, filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt, };

use crate::ui::tile_factory::WebContentFactory;
use crate::session::*;
use crate::route::admit_route::AdmitRoute;
use crate::route::default_route::BasicRoute;
use crate::route::home_route::HomeRoute;
use crate::route::intervention_route::InterventionRoute;
use crate::route::intervention_details_route::InterventionDetailsRoute;
use crate::route::login_route::LoginRoute;
use crate::route::patient_route::PatientRoute;
use crate::route::nle_route::*;

// TODO: ideally we'd use an external session store, not just cookies. Until the application is largely working, we'll have to leave this for now. //storage::RedisSessionStore}; 
mod constants;
mod dto;
mod ui;
mod dao;
mod nle;
mod route;
mod session;

/// # Main program
///  Loads the NLP engine and adds handlers for key paths of the web application
/// 
/// ### References: Add CORS headers to allow javascript connectivity
///  https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html 
/// 
/// Returns std::io::Result<()> 
/// 
#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logging();
    let config = SysConfig::new(env::var(constants::CARGO_MANIFEST_DIR));
    let binding_addr = config.clone().website_bind_address;

    //establish database connection for entire application here, add to the application session
    let db_url = &config.db_conn_str.clone();

    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("Database connection established to: http://{}", db_url);
            pool
        },
        Err(e) => {
            tracing::error!("{}", e);
            panic!("{}", e)
        },
    };

 /* let nle_session: ort::session::Session = Session::builder().expect("Session could not be established")
                  .with_optimization_level(GraphOptimizationLevel::Level1).expect("No Session")
                  .with_intra_threads(1).expect("Insufficient threads")
                  .commit_from_file(&(config.model_data_dir.clone() + &config.language_model_file.clone()) ).expect("File could not be accessed");
  let shared_session = Arc::new(nle_session);
*/
    println!("MapleHMS is running! Access via: http://{}", binding_addr.clone());

    let static_path_base = std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\webc\\static\\";

    // use the Builder pattern to add one route at a time
    HttpServer::new( move || {
        let tmp_app_key = config.get_application_secret_key(); //  MUST create within the enclosure, but NOT below, to have it available and consistent for the two uses below

        App::new()
            .wrap(
            Cors::default()
                //.allowed_origin("http://localhost:8000") // Restrict to specific origin
                .allow_any_origin() // not great... will have to do for now
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
                .allow_any_header()
                .max_age(3600),
        )
        .app_data(  // this enclosure allows the session state to be created and made available to all routes. actix_web magic.
            web::Data::new( session::AppSession {
                    wcf: WebContentFactory::new(&static_path_base, config.app_version.clone()),
                    app_key: tmp_app_key.clone(),
                    connection: db_pool.clone(),
                    system_config: config.clone()//,
                }
            ) 
        )
        .wrap(SessionMiddleware::new(CookieSessionStore::default(), tmp_app_key.clone())) // for user session
        .route("/", web::get().to( BasicRoute::default_route ))
        .route("/login", web::post().to( LoginRoute::login ))
        .route("/home", web::get().to( HomeRoute::route_to_home )) // main workspace
        .route("/patientdtls", web::post().to( PatientRoute::route_to_patient_details ))
        .route("/nlprompt", web::post().to( NLERoute::natural_language_prompt ))
        .route("/admit", web::post().to( AdmitRoute::route_to_admit_discharge ))
        .route("/admitnew", web::post().to( AdmitRoute::route_to_admit_new_no_patient ))
        .route("/admitsave", web::post().to( AdmitRoute::route_to_admit_save ))
        .route("/discharge", web::post().to( AdmitRoute::route_to_discharge_patient ))
        .route("/dischargesave", web::post().to( AdmitRoute::route_to_discharge_patient_save ))
        .route("/intvlink", web::post().to( InterventionRoute::route_to_modify_intervention_basic ))
        .route("/intvnew", web::post().to( InterventionRoute::route_to_add_new_intervention ))
        .route("/intv", web::post().to( InterventionRoute::route_to_view_or_modify_intervention ))
        .route("/intvsave", web::post().to( InterventionRoute::route_to_intervention_save ))
        .route("/intvdtlnew", web::post().to( InterventionDetailsRoute::route_to_add_intervention_detail ))
        .route("/intvdtlsave", web::post().to( InterventionDetailsRoute::route_to_intervention_detail_save ))
        .route("/isItUp", web::get().to( BasicRoute::is_it_up_route ))
        .route("/logout", web::get().to( LoginRoute::logout ))
        .default_service(web::to(BasicRoute::not_found_route))
        .service(Files::new("/webc/", "./webc"))  // ref: ttps://actix.rs/docs/static-files/
    })
    .bind(binding_addr)? //  "127.0.0.1:8000"
    .run()
    .await
}

/// ### fn init_logging()
///   Initializes standard Rust logging for the application, creating a file with name format: maple_hms-%Y-%b-%d_%H%M%S.log
/// 
/// #### Parameters: None
/// 
/// #### Returns: None
/// 
fn init_logging(){
    // added per recommendation from 0-to-Prod
    // https://rust.code-maven.com/logging/tracing-to-a-file.html
    //
    let log_filename = "maple_hms-".to_owned() + &chrono::Local::now().format("%Y-%b-%d_%H%M%S").to_string() +".log";
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_writer(
                        std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(log_filename)
                            .unwrap(),
                    )
                    .with_filter(LevelFilter::DEBUG),
            )
            .init();
    tracing::info!("MapleHMS is running!");
}