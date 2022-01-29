use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestURL {
  origin_url: String,
  custom_url: String,
  // expired_date: String,
}

#[derive(Serialize, Debug)]
pub struct ResponseURL {
  origin_url: String,
  hashed_url: String,
  custom_url: String,
  // expired_date: String,
}

/// extract `Info` using serde
pub async fn shorten_url(req: web::Json<RequestURL>) -> Result<impl Responder> {
  let mut res = ResponseURL {
    origin_url: req.origin_url.clone(),
    hashed_url: "".to_string(),
    custom_url: "".to_string(),
    // expired_date: String::from(""),
  };

  if req.custom_url == "" {
    let hash: String = hash_url(req.origin_url.clone()); 
    res.origin_url = hash;
  } else {
    res.custom_url = req.custom_url.clone();
  }
  return Ok(web::Json(res))
}

pub fn hash_url(url: String) -> String {
  // create hashing to str
  let split_url = url.split('/').nth(2).unwrap().chars();
  let first_char: String = split_url.clone().nth(0).unwrap().to_string(); 
  let char_len: usize = split_url.clone().count(); //find the length of the main URL
  let last_char: String = split_url.clone().nth(char_len - 1).unwrap().to_string();
  let str_id: String = first_char + &last_char;

  // create hashing to number
  let mut hasher = DefaultHasher::new();
  url.hash(&mut hasher);
  let num_id = hasher.finish().to_string();
  let slice = &num_id[0..4];

  let final_hash = str_id + &slice;

  return String::from(&final_hash);
}
