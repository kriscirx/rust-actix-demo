use actix_web::{web, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  
  let host = env::var("HOST").expect("HOST is not set.");

  let port = env::var("PORT").expect("PORT is not set.");

  let server = HttpServer::new(|| {
    App::new()
      .route("/hello", web::get().to(cargo_sample::manual_hello))
  })
  .bind(format!("{}:{}", host, port))
  .unwrap()
  .run();

  eprintln!("Listening on {}:{}", host, port);

  server.await
}
