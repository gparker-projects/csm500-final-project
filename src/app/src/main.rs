use actix_web::{web, App, HttpRequest, HttpServer, Responder};

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

/// performs an execution of the NLP engine
/// 
/// check by going to: http://127.0.0.1:8000/nlp
/// 
async fn machine_learn_test(_req: HttpRequest) -> impl Responder {
  let store = nlp::NLP{}.execute();
  
  format!("Rust POC WebDBML2! {}", store.await.to_string())
}

/// Basic landing page type response
/// 
/// check by going to: http://127.0.0.1:8000/
///
async fn welcome(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Rust POC webDB {}!", &name)
}

/// performs a connect to the database
/// 
/// check by going to: http://127.0.0.1:8000/db
/// 
async fn db(_req: HttpRequest) -> impl Responder {
  let store = auth_objects::AuthObjects::new("postgres://postgres:csm500@localhost:5432/csm500").await;

  let users = store.get_users( Some(10), 0).await.expect("May be no users");

  let mut s = String::new();
  for row in users.iter() {                          // row: &User
      s = s + &row.id.to_string() + " " + &row.username + "; ";
  }
  format!("Users: {}, #{}", s, users.len())
}

/// # Main program
/// 
/// Loads the NLP engine and adds handlers for key paths of the web application
/// 
/// Returns std::io::Result<()> for 
#[tokio::main]
async fn main() -> std::io::Result<()> {

  HttpServer::new(|| {
  App::new()
    .route("/", web::get().to( welcome ))
    .route("/ml", web::get().to( machine_learn_test ))
    .route("/db", web::get().to( db ))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}