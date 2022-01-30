use actix_web::{web, App, HttpServer};

mod shorten;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(shorten::main)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
