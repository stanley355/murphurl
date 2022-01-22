use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestURL {
  origin_url: String,
  hashed_url: String,
  custom_url: String,
  expired_date: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseURL {
  origin_url: String,
  hashed_url: String,
  custom_url: String,
  expired_date: String,
}

/// extract `Info` using serde
pub async fn shorten_url(req: web::Json<RequestURL>) -> Result<impl Responder> {
  let res = ResponseURL {
    origin_url: req.origin_url.clone(),
    hashed_url: String::from(""),
    custom_url: String::from(""),
    expired_date: String::from(""),
  };
  Ok(web::Json(res))
}
