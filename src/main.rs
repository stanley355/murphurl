use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;

mod shorten;

async fn pg_client() -> postgres::Client {
  dotenv().ok();

  // Create Ssl postgres connector without verification as required to connect to Heroku.
  let mut ssl = SslConnector::builder(SslMethod::tls()).unwrap();
  ssl.set_verify(SslVerifyMode::NONE);
  let tls = MakeTlsConnector::new(ssl.build());

  let client = postgres::Client::connect(&env::var("PG_URL").expect("Fail to found PG URL"), tls);
  return client.unwrap();
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = &env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = &env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);

    let pg_client = web::Data::new(pg_client());

    HttpServer::new(move || {
        App::new()
        .app_data(pg_client.clone())
        .route("/api/v1", web::post().to(shorten::shorten_url)) //to create shorten URL
        .route("/api/v1/{url}", web::get().to(shorten::find_redirect_url)) //to find origin URL  
    })
    .bind(address)?
    .run()
    .await
}
