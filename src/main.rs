use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod shorten;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let address = format!(
        "{}:{}",
        env::var("HOST").unwrap(),
        &env::var("PORT").unwrap()
    );

    // to do: use app scope
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/v1", web::post().to(shorten::shorten_url)) //to create shorten URL
                .route("/v1/{url}", web::get().to(shorten::find_redirect_url)), //to find origin URL
        )
    })
    .bind(address)?
    .run()
    .await
}
