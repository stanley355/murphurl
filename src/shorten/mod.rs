use actix_web::{web, Responder, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod db;
mod structs;

/// extract `Info` using serde
pub async fn main(req: web::Json<structs::RequestURL>) -> Result<impl Responder> {
  db::create_table().expect("Failed to create table");

  let res = structs::ResponseURL {
    origin_url: req.origin_url.clone(),
    hashed_url: hash_url(&req.origin_url),
    custom_url: req.custom_url.clone(),
  };

  let url_data = check_existing_data(res);

  return Ok(web::Json(url_data));
}

pub fn hash_url(url: &String) -> String {
  // create url identifier with the first and last char of the basepath
  let mut split_url = url.split('/').nth(2).unwrap().chars();
  let first_char: String = split_url.clone().nth(0).unwrap().to_string();
  let char_len: usize = split_url.clone().count(); //find the length of the main URL
  let last_char: &String = &split_url.nth(char_len - 1).unwrap().to_string();
  let str_id: String = first_char + &last_char;

  // create random number and slice the first to fourth chars
  let mut hasher = DefaultHasher::new();
  url.hash(&mut hasher);
  let num_id = &hasher.finish().to_string();

  let final_hash = str_id + &num_id[0..4];

  return String::from(&final_hash);
}

fn check_existing_data(mut res: structs::ResponseURL) -> structs::ResponseURL {
  let db_data = db::check_url_data(res.clone()).expect("Fail to check");
  if db_data.origin_url == res.origin_url {
    res = db_data;
  } else {
    db::insert_url_data(res.clone()).expect("Failed to insert url data");
  }

  return res;
}
