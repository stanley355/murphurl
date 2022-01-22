use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestURL {
  origin_url: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseURL {
  origin_url: String,
}

/// extract `Info` using serde
pub async fn shorten_url(req: web::Json<RequestURL>) -> Result<impl Responder> {
  let mut hasher = DefaultHasher::new();
  req.origin_url.hash(&mut hasher);
  
  let res = ResponseURL {
    origin_url: hasher.finish().to_string(),
  };
  Ok(web::Json(res))
}
