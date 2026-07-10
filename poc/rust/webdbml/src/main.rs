use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// https://github.com/LukeMathWalker/zero-to-production
// https://github.com/launchbadge/sqlx

//  B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
// https://github.com/Rust-Web-Development/code  

mod auth_objects;

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
    //.route("/{name}", web::get().to( greet ))
    .route("/db", web::get().to( db ))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await

}