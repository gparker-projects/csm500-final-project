use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// https://github.com/LukeMathWalker/zero-to-production
// https://github.com/launchbadge/sqlx

//  B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
// https://github.com/Rust-Web-Development/code  

async fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Rust POC webDB {}!", &name)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

  let store = store::Store::new("postgres://postgres:csm500@localhost:5432/csm500").await;

  HttpServer::new(|| {
  App::new()
    .route("/", web::get().to( greet ))
    .route("/{name}", web::get().to( greet ))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await

}