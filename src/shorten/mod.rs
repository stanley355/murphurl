use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize};

use dotenv::dotenv;
use std::env;

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres::{Client};
use postgres_openssl::MakeTlsConnector;

mod db;
mod structs;
mod utils;


#[derive(Deserialize, Debug)]
pub struct RequestURL {
  origin_url: String,
  custom_url: String,
}


// Create shortened url
pub async fn shorten_url(req: web::Json<RequestURL>) -> Result<HttpResponse, actix_web::Error> {

  let res = Box::new(structs::ResponseURL {
    origin_url: req.origin_url.clone(),
    hashed_url: utils::hash_url(&req.origin_url),
    custom_url: req.custom_url.clone(),
  });

  let url_data = check_existing_origin(res);

  return Ok(HttpResponse::Ok().json(url_data));
}


fn check_existing_origin(mut res: Box<structs::ResponseURL>) -> Box<structs::ResponseURL> {
  let db_data = db::check_existing_url(res.clone()).expect("Fail to check");
  
  if db_data.origin_url == res.origin_url {
    res = db_data;
  } else if res.hashed_url == "" {
    res = res; //if hashed_url has empty string
  } else {
    db::insert_new_url(res.clone()).expect("Failed to insert url data");
  }

  return res;
}

// Check existing shortened url
pub async fn find_shorten_url(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
  let url = req.match_info().get("url");

  let res = Box::new(structs::ResponseURL {
    origin_url: "".to_string(),
    hashed_url: url.unwrap().to_string(),
    custom_url: url.unwrap().to_string(),
  });

  let db_data = db::check_existing_url(res.clone()).expect("Fail to check");

  return Ok(HttpResponse::Ok().json(db_data));
}
