use actix_web::{web, App, HttpServer};

mod shorten;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{url}", web::get().to(shorten::find_shorten_url))
            .route("/", web::post().to(shorten::shorten_url))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
