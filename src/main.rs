use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(cargo_sample::manual_hello))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
