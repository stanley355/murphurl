use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod shorten;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let host = &env::var("HOST").unwrap_or("127.0.0.1".to_string());
  let port = &env::var("PORT").unwrap_or("8080".to_string());
  let address = format!("{}:{}", host, port);

  HttpServer::new(|| {
    App::new()
      .route(
        "/api/v1",
        web::post().to(shorten::route_handler::shorten_url),
      )
      .route(
        "/api/v1/{url}",
        web::get().to(shorten::route_handler::find_redirect_url),
      )
      .route(
        "/api/v1/migrate",
        web::post().to(shorten::route_handler::migrate_db),
      )
      .route(
        "/api/v2",
        web::post().to(shorten::route_handler::bulk_upload),
      )
  })
  .bind(address)?
  .run()
  .await
}
