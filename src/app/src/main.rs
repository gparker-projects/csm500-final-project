//use std::ptr::null;

use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder, web::Redirect};
use actix_cors::Cors;
//use serde::Deserialize; //pg 51 of ZeroToProd
///
/// # Main program executable for the project
///
///      CSM500 Project (April - October 2026)
///         Graham Parker (Student ID: 240120522)
/// 
/// REFERENCES
/// 
/// Refs for Web and DB:
/// [1] B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
/// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
/// https://github.com/Rust-Web-Development/code
/// 
/// Refs for ML code:
/// [2] S. Lyu and A. Rzeznik, Practical Rust Projects: Build Serverless, AI, Machine Learning, Embedded, Game, and Web Applications. Berkeley, CA: Apress, 2023. doi: DOI:%2010.1007/978-1-4842-9331-7.
/// https://github.com/LukeMathWalker/zero-to-production
///
mod auth_objects;
mod nlp;
mod errors;

// application-wide database string; should come from a configurable parameter file (TODO)
const DB_CONN_STR: &str = "postgres://postgres:csm500@localhost:5432/csm500";

#[derive(serde::Deserialize)]
pub struct LoginFormData {
    #[serde(rename = "mplUsername")]
    username: String,
    #[serde(rename = "mplPassword")]
    password: String,
}

#[derive(serde::Deserialize)]
pub struct NLPromptFormData {
    #[serde(rename = "prompt")]
    prompt: String,
}



/// performs a connect to the database
/// 
/// check by going to: http://127.0.0.1:8000/db
/// 
async fn login(req: web::Form<LoginFormData>) -> impl Responder {
    
  let cur_db_conn = auth_objects::AuthObjects::new(DB_CONN_STR).await;
  let user_can_login = cur_db_conn.can_user_login(req.username.clone(), req.password.clone()).await.expect( &errors::DatabaseError::NotFoundError.to_string() );

  if user_can_login {
    HttpResponse::Ok().body( format!("User can login: {}", req.username) )
    //Redirect::to("").permanent()
  }
  else{
    HttpResponse::Ok().body( format!("Login denied for {}", req.username) )
    //Redirect::to("/login").permanent()
  }
}

/// performs a natural language prompt using the built in engine
/// 
/// check by going to: http://127.0.0.1:8000/db
/// 
async fn natural_language_prompt(req: web::Form<NLPromptFormData>) -> impl Responder {
    println!("Received prompt for: {}", req.prompt);

    HttpResponse::Ok().body( format!("NL Response: {}", req.prompt) )
}


/// performs an execution of the NLP engine
/// 
/// check by going to: http://127.0.0.1:8000/ml
///                    http://localhost:8000/ml
/// 
async fn machine_learn_test(_req: HttpRequest) -> impl Responder {
  let store = nlp::NLP{}.execute();
  
  HttpResponse::Ok().body(format!("<html><body><b>Rust</b> POC WebDBML2! {}</body></html>", store.await.to_string())) 
}

/// performs a connect to the database
/// 
/// check by going to: http://127.0.0.1:8000/db 
///                    http://localhost:8000/db
/// 
async fn db(_req: HttpRequest) -> impl Responder {
  // TODO: use a pool instead, as this will block another query/result in multiple connections the DB may not be able to accomodate
  let cur_db_conn = auth_objects::AuthObjects::new(DB_CONN_STR).await;
  let users = cur_db_conn.get_users( Some(10), 0).await.expect( &errors::DatabaseError::NotFoundError.to_string() );

  let mut s = String::new();
  for row in users.iter() {                          // row: &User
      s = s + &row.id.to_string() + " " + &row.username + "; ";
  }
  HttpResponse::Ok().body(format!("Users: {}, #{}", s, users.len()))
}

/// Allows a monitoring services to perform a basic "is the application up?" check
/// 
async fn is_it_up() -> impl Responder {
  HttpResponse::Ok().body("MapleEMR is Up")
}


/// Main workspace page of the application, to be supplemented with lots of Javascript, CSS and API calls
/// 
async fn workspace() -> impl Responder {
  HttpResponse::Ok().body("MapleEMR Workspace")
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

  // use the Builder pattern to add one route at a time
  HttpServer::new(|| {
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
        .route("/", web::get().to( login ))
        .route("/login", web::post().to( login ))
        .route("/maple", web::post().to( workspace )) // main workspace
        .route("/nlprompt", web::post().to( natural_language_prompt ))
        .route("/ml", web::get().to( machine_learn_test ))
        .route("/db", web::get().to( db ))
        .route("/isItUp", web::get().to( is_it_up ))    
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}