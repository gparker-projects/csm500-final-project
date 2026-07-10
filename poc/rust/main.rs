use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// REF: [1]L. Palmieri, Zero to production in Rust: an opinionated introduction to backend development. Brétigny-sur-Orge, FR: Amazon, 2025.

async fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
    App::new()
    .route("/", web::get().to(greet))
    .route("/{name}", web::get().to(greet))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}