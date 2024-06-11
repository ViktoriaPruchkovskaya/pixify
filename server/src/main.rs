use actix_web::{App, HttpServer};
use pixify::api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::services))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
