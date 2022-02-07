use actix_web::{web, HttpRequest, HttpResponse, Result};

use dotenv::dotenv;
use std::env;

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres::Client;
use postgres_openssl::MakeTlsConnector;

mod response;

fn connect_pg() -> Result<Box<Client>, postgres::Error> {
  dotenv().ok();

  // Create Ssl postgres connector without verification as required to connect to Heroku.
  let mut ssl = SslConnector::builder(SslMethod::tls()).unwrap();
  ssl.set_verify(SslVerifyMode::NONE);
  let tls = MakeTlsConnector::new(ssl.build());

  let client = Box::new(Client::connect(&env::var("PG_URL").unwrap(), tls)?);
  return Ok(client);
}

// Create shortened url
pub async fn shorten_url(
  req: web::Json<response::RequestURL>,
) -> Result<HttpResponse, actix_web::Error> {

  let pg_client = connect_pg().expect("Failed to connect to database");
  let res = Box::new(response::ResponseURL {
    origin_url: req.origin_url.clone(),
    hashed_url: response::hash_url(&req.origin_url),
    custom_url: req.custom_url.clone(),
  });

  let url_data = res.verify_and_hash(pg_client).unwrap();

  return Ok(HttpResponse::Ok().json(url_data));
}

pub async fn find_redirect_url(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
  let pg_client = connect_pg().expect("Failed to connect to database");
  let short_url = req.match_info().get("url");
  
  let res = Box::new(response::ResponseURL {
    origin_url: "".to_string(),
    hashed_url: short_url.unwrap().to_string(),
    custom_url: short_url.unwrap().to_string(),
  }); 

  let url_data = res.fetch_origin_url(pg_client).unwrap();

  return Ok(HttpResponse::Ok().json(url_data));
}
