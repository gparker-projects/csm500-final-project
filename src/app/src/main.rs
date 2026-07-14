use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// https://github.com/LukeMathWalker/zero-to-production
// https://github.com/launchbadge/sqlx

//  B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
// https://github.com/Rust-Web-Development/code  


// Refs for Web and DB:
// [1] B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
// https://github.com/Rust-Web-Development/code  

// Refs for ML code:
// [2] S. Lyu and A. Rzeznik, Practical Rust Projects: Build Serverless, AI, Machine Learning, Embedded, Game, and Web Applications. Berkeley, CA: Apress, 2023. doi: DOI:%2010.1007/978-1-4842-9331-7.
//

mod auth_objects;
mod nlp;

// check by going to: http://127.0.0.1:8000/db
async fn machine_learn_test(_req: HttpRequest) -> impl Responder {
  let store = nlp::NLP{}.execute();
  
  format!("Rust POC WebDBML2! {}", store.await.to_string())
}

async fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Rust POC webDB {}!", &name)
}

async fn db(_req: HttpRequest) -> impl Responder {
  let store = auth_objects::AuthObjects::new("postgres://postgres:csm500@localhost:5432/csm500").await;

  let users = store.get_users( Some(10), 0).await.expect("May be no users");

  let mut s = String::new();
  for row in users.iter() {                          // row: &User
      s = s + &row.id.to_string() + " " + &row.username + "; ";
  }
  format!("Users: {}, #{}", s, users.len())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

  HttpServer::new(|| {
  App::new()
    .route("/", web::get().to( greet ))
    .route("/ml", web::get().to( machine_learn_test ))
    .route("/db", web::get().to( db ))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}