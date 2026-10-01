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

use actix_web::http::StatusCode;

use actix_cors::Cors;
use actix_files::*;
use actix_session::{storage::CookieSessionStore, SessionMiddleware}; //, storage::RedisSessionStore}
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::cookie::Key;
use std::env;
use std::fs::read_to_string;
use sqlx::postgres::{PgPoolOptions};
use tracing;
use tracing_subscriber::{
    Layer, filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::ui::tile_factory::WebContentFactory;
use crate::session::*;
use crate::route::admit_route::AdmitRoute;
use crate::route::default_route::DefaultRoute;
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

/// ### fn is_it_up()
/// 
/// Allows a monitoring services to perform a basic "is the application up?" check
/// 
/// #### Parameters: None
/// 
/// #### Returns: 
/// * Responder: the general responder that allows the system to report system is up using an Ok() response
/// 
async fn is_it_up() -> impl Responder {
    tracing::info!("-> /isItUp Requested");
    HttpResponse::Ok().body("MapleEMR is Up")
}

/// ### fn get_static_path_base()
/// 
/// Helper function: obtains the web static path base, which is used to retrieve many sources of static content
/// TODO: if this is not being used anywhere other than WebContentFactory, can we remove it?
/// 
/// #### Parameters: None
/// 
/// #### Returns: 
/// * String: the base static file path, based on std::env::current_dir()
/// 
fn get_static_path_base() -> String{
   //let path = see below
   //println!("Default Route base dir: {}", path.clone());
   return std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\webc\\static\\";
}

/// ### fn get_application_secret_key()
///   Provides the secret key for the application, usually from a config file (TODO)
/// 
/// #### Referencees:
///  https://docs.rs/actix-web/latest/actix_web/cookie/struct.Key.html
/// 
/// #### Parameters: None
/// 
/// #### Returns: 
/// * Key: the Key obtained from the actix_web::cookie::Key class
/// 
fn get_application_secret_key() -> Key {
    tracing::info!(">get_application_secret_key()");

    actix_web::cookie::Key::from(
    std::env::var("SESSION_KEY")
        .unwrap_or_else(|_| "this_is_a_new_system_key_to_prevent_regeneration_of_a_key_every_time_the_app_starts".to_string())
        .as_bytes()
    )
}

/// ### fn init_logging()
///   Initializes standard Rust logging for the application, creating a file with name format: maple_emr-%Y-%b-%d_%H%M%S.log
/// 
/// #### Parameters: None
/// 
/// #### Returns: None
/// 
fn init_logging(){
  // added per recommendation from 0-to-Prod
  // https://rust.code-maven.com/logging/tracing-to-a-file.html
  //
  let log_filename = "maple_emr-".to_owned() + &chrono::Local::now().format("%Y-%b-%d_%H%M%S").to_string() +".log";
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
        // Enable this to also log to STDOUT:
        //.with(tracing_subscriber::fmt::layer())
        .init();
  tracing::info!("MapleEMR is running!");
  println!("MapleEMR is running! Access via: http://127.0.0.1:8000");
}

/// ### init_config()
///   Reads the system configuration file from a static path... so it is the only one we need to do this from
///  the rest of the config settings are in this config file, eliminating many constants otherwise requird by the application
/// 
/// #### Parameters: None
/// 
/// #### Returns:
/// * SysConfig: an initialized SysConfig instance
/// 
fn init_config() -> SysConfig {
    // collect the cargo manifest directory at runtime, which means it might not be present
    let cargo_manifest_dir = match env::var(constants::CARGO_MANIFEST_DIR) {
        Ok(tmp_path) => {
            tracing::info!("CARGO_MANIFEST_DIR = {}", tmp_path);
            tmp_path
        }
        Err(e) => {
            tracing::error!("CARGO_MANIFEST_DIR not set: {}", e);
            "INVALID_PATH".to_string()
        }
    };

    let base_model_data_dir = cargo_manifest_dir.clone()  + constants::DATA_SUB_DIRECTORY;
    let toml_config_file = cargo_manifest_dir.clone()  + constants::SYSTEM_CONFIGURATION_FILE;

    let toml_config_str = read_to_string(toml_config_file.clone()); 
    let mut final_config: session::SysConfig = Default::default();

    match toml_config_str {
        Ok(results) => {

            let tmp_config = toml::from_str::<session::SysConfig>( &results );
            match tmp_config {
                Ok(ok_config) => {
                    tracing::info!("Configuration loaded: {}", toml_config_file.clone());
                    final_config = ok_config;
                }
                Err(e) => {
                    tracing::error!("Error reading from TOML ({}): {}", toml_config_file.clone(), e);
                    eprintln!("Error reading from TOML ({}): {}", toml_config_file.clone(), e);
                },
            }
        }
        Err(e) => {
            tracing::error!("Error reading from TOML ({}): {}", toml_config_file.clone(), e);
            eprintln!("Error reading from TOML ({}): {}", toml_config_file.clone(), e);
        },
    }
    final_config.cargo_manifest_dir = cargo_manifest_dir; // override some of the values, with setting obtained elsewhere in by the system
    final_config.model_data_dir = base_model_data_dir;

    final_config
}

/// ### route_to_not_found()
///   Route for processing resource not found / 404 errors
/// 
/// #### Parameters: None
/// 
/// #### Returns:
/// * Responder (actix_web::response::responder): the HTTP responder (response) for the request
/// 
async fn route_to_not_found() -> impl Responder {
    //HttpResponse::NotFound().body("Sorry, Page not found")
     actix_web::web::Redirect::to("/home").using_status_code(StatusCode::SEE_OTHER)
}

 
/// # Main program
/// 
/// Loads the NLP engine and adds handlers for key paths of the web application
/// 
/// Ref: Add CORS headers to allow javascript connectivity
///      ->  https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html 
/// 
/// Returns std::io::Result<()> for 
#[tokio::main]
async fn main() -> std::io::Result<()> {
  init_logging();
  let config = init_config();
  let binding_addr = config.clone().website_bind_address;

  //establish database connection for entire application here, add to the application session
  let db_url = &config.db_conn_str.clone();

  let db_pool = match PgPoolOptions::new()
      .max_connections(5)
      .connect(db_url)
      .await
  {
      Ok(pool) => {
        tracing::info!("Database connection established to: {}", db_url);
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

  // use the Builder pattern to add one route at a time
  HttpServer::new( move || {

    let tmp_app_key = get_application_secret_key(); // create within the enclosure to make sure it is available and consistent for the two uses below
    

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
                  //wcf: Mutex::new( WebContentFactory::new(&get_static_path_base()) )
                  wcf: WebContentFactory::new(&get_static_path_base(), config.app_version.clone()),
                  app_key: tmp_app_key.clone(),
                  connection: db_pool.clone(),
                  system_config: config.clone()//,
                  //nle_session: Arc::clone(&shared_session), 
              }
          ) 
      )
      .wrap(SessionMiddleware::new(CookieSessionStore::default(), tmp_app_key.clone())) // for user session
      .route("/", web::get().to( DefaultRoute::default_route ))
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
      .route("/isItUp", web::get().to( is_it_up ))
      .route("/logout", web::get().to( LoginRoute::logout ))
      .default_service(web::to(route_to_not_found))
      .service(Files::new("/webc/", "./webc"))  // ref: ttps://actix.rs/docs/static-files/
  })
  .bind(binding_addr)? //  "127.0.0.1:8000"
  .run()
  .await
}