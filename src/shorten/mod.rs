use actix_web::{web, HttpRequest, HttpResponse, Result};

mod db;
mod model;
mod utils;

// Create shortened url
pub async fn shorten_url(
  req: web::Json<model::ShortURL>,
) -> Result<HttpResponse, actix_web::Error> {
  let res = Box::new(model::ShortURL {
    origin_url: req.origin_url.clone(),
    hashed_url: utils::hash_url(&req.origin_url),
    custom_url: req.custom_url.clone(),
  });

  let url_data = res.verify_and_hash().unwrap();

  return Ok(HttpResponse::Ok().json(url_data));
}

pub async fn find_origin_url(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
  let short_url = req.match_info().get("url");
  let res = Box::new(model::ShortURL {
    origin_url: "".to_string(),
    hashed_url: short_url.unwrap().to_string(),
    custom_url: short_url.unwrap().to_string(),
  });

  let url_data = res.fetch_origin_url().unwrap();

  return Ok(HttpResponse::Ok().json(url_data));
}

pub async fn migrate_db() -> Result<HttpResponse, actix_web::Error> {
  db::create_db().unwrap();
  return Ok(HttpResponse::Ok().body("Ok"));
}
