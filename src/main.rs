use actix_web::{web, App, HttpServer};

mod shorten;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::connect_pg();
    HttpServer::new(|| App::new().route("/", web::post().to(shorten::shorten_url)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
