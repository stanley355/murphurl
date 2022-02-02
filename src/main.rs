use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod shorten;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .route("/v1/{url}", web::get().to(shorten::find_shorten_url))
            .route("/v1", web::post().to(shorten::shorten_url))
    })
    .bind(&env::var("HOST").unwrap())?
    .run()
    .await
}
