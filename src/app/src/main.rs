//! # Main program executable for the project
//!
//!      CSM500 Project (April - October 2026)
//!         Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! Refs for Web and DB:
//! [1] B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
//! https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
//! https://github.com/Rust-Web-Development/code
//! 
//! Refs for ML code:
//! [2] S. Lyu and A. Rzeznik, Practical Rust Projects: Build Serverless, AI, Machine Learning, Embedded, Game, and Web Applications. Berkeley, CA: Apress, 2023. doi: DOI:%2010.1007/978-1-4842-9331-7.
//! https://github.com/LukeMathWalker/zero-to-production
//!

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
//use actix_web::http::StatusCode;
use actix_web::cookie::Key;
use actix_cors::Cors;
use actix_files::*;
use actix_session::{storage::CookieSessionStore, SessionMiddleware}; //, storage::RedisSessionStore} // for user session management: https://docs.rs/actix-session/latest/actix_session/

use crate::webc::web_content::WebContentFactory; 
use crate::route::admit_route::AdmitRoute;
use crate::route::default_route::DefaultRoute;
use crate::route::home_route::HomeRoute;
use crate::route::intervention_route::InterventionRoute;
use crate::route::login_route::LoginRoute;
use crate::route::patient_route::PatientRoute;
use crate::route::nlp_route::NLPRoute;

// TODO: ideally we'd use an external session store, not just cookies. Until the application is largely working, we'll have to leave this for now. //storage::RedisSessionStore}; 
mod constants;
mod dto;
mod webc;
mod dao;
mod nlp;
mod route;
mod session;

//use std::sync::Mutex; // needed for thread safety per https://actix.rs/docs/application/

///
/// Allows a monitoring services to perform a basic "is the application up?" check
/// 
async fn is_it_up() -> impl Responder {
  println!("-> /isItUp Requested");
  HttpResponse::Ok().body("MapleEMR is Up")
}

///
/// Helper function: obtains the web static path base, which is used to retrieve many sources of static content
/// TODO: if this is not being used anywhere other than WebContentFactory, can we remove it?
/// 
fn get_static_path_base() -> String{
   //let path = see below
   //println!("Default Route base dir: {}", path.clone());
   return std::env::current_dir().expect("Base path to executable could not be found").display().to_string() + "\\webc\\static\\";
}

///
/// Provides the secret key for the application, usually from a config file (TODO)
/// REF: https://docs.rs/actix-web/latest/actix_web/cookie/struct.Key.html
/// 
fn get_application_secret_key() -> Key {
    println!(">get_application_secret_key()");

    actix_web::cookie::Key::from(
    std::env::var("SESSION_KEY")
        .unwrap_or_else(|_| "this_is_a_new_system_key_to_prevent_regeneration_of_a_key_every_time_the_app_starts".to_string())
        .as_bytes()
    )
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

//  //nlp::NLP{}.execute();
// https://docs.rs/actix-cors/latest/actix_cors/struct.Cors.html 

  println!("MapleEMR is running! Access via: http://127.0.0.1:8000");

  // use the Builder pattern to add one route at a time
  HttpServer::new(|| {

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
                app_version: "v1.0".to_string(),
                //wcf: Mutex::new( WebContentFactory::new(&get_static_path_base()) )
                wcf: WebContentFactory::new(&get_static_path_base()),
                app_key: tmp_app_key.clone()
              }
            ) 
        )
        .wrap(SessionMiddleware::new(CookieSessionStore::default(), tmp_app_key.clone())) // for user session
        .route("/", web::get().to( DefaultRoute::default_route ))
        .route("/login", web::post().to( LoginRoute::login ))
        .route("/home", web::get().to( HomeRoute::route_to_home )) // main workspace
        .route("/patientdtls", web::post().to( PatientRoute::route_to_patient_details ))
        .route("/nlprompt", web::post().to( NLPRoute::natural_language_prompt ))
        .route("/admit", web::post().to( AdmitRoute::route_to_admit_discharge ))
        .route("/admitnew", web::post().to( AdmitRoute::route_to_admit_new_no_patient ))
        .route("/admitsave", web::post().to( AdmitRoute::route_to_admit_save ))
        .route("/discharge", web::post().to( AdmitRoute::route_to_admit_discharge ))
        .route("/modintv", web::post().to( InterventionRoute::route_to_modify_intervention ))
        .route("/isItUp", web::get().to( is_it_up ))
        .service(Files::new("/webc/", "./webc"))  // ref: ttps://actix.rs/docs/static-files/
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}