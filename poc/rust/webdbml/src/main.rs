use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// https://github.com/LukeMathWalker/zero-to-production
// https://github.com/launchbadge/sqlx

// Refs for Web and DB:
// [1] B. Gruber, Rust web development: with Warp, Tokio, and Reqwest. Shelter Island, NY: Manning Publications Co, 2023.
// https://learning.oreilly.com/library/view/rust-web-development/9781617299001/OEBPS/Text/07.htm#sigil_toc_id_85
// https://github.com/Rust-Web-Development/code  

// Refs for ML code:
// [2] S. Lyu and A. Rzeznik, Practical Rust Projects: Build Serverless, AI, Machine Learning, Embedded, Game, and Web Applications. Berkeley, CA: Apress, 2023. doi: DOI:%2010.1007/978-1-4842-9331-7.
//    

mod nlp;

// check by going to: http://127.0.0.1:8000/db
async fn machine_learn_test(_req: HttpRequest) -> impl Responder {
  let store = nlp::NLP{}.execute();
  
  format!("Rust POC WebDBML2! {}", store.await.to_string())
}





#[tokio::main]
async fn main() -> std::io::Result<()> {

  HttpServer::new(|| {
  App::new()
    .route("/", web::get().to( machine_learn_test ))
    .route("/ml", web::get().to( machine_learn_test ))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await

}