use actix_web::{web, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  
  let ip = env::var("IP")
    .unwrap_or_else(|_| "0.0.0.0".to_string());

  let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("PORT must be a number");

  HttpServer::new(|| {
    App::new()
      .route("/hello", web::get().to(cargo_sample::manual_hello))
  })
  .bind((ip, port))?
  .run()
  .await
}
