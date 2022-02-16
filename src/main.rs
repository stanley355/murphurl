use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;
use dotenv::dotenv;
use std::env;

mod shorten;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pg_url = &env::var("PG_URL").unwrap();
    let host = &env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = &env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);

    let pg_pool = match PgPool::connect(pg_url).await {
        Ok(pool) => Box::new(pool),
        Err(err) => panic!("Can't connect to PG POOL: {}", err),
    };

    HttpServer::new(move || {
        App::new()
        .app_data(pg_pool.clone())
        .route("/v1", web::post().to(shorten::shorten_url)) //to create shorten URL
        .route("/v1/{url}", web::get().to(shorten::find_redirect_url)) //to find origin URL  
    })
    .bind(address)?
    .run()
    .await
}
