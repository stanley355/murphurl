use crate::shorten::{db, model, utils};
use actix_web::{web, HttpRequest, HttpResponse, Result};

// Create shortened url
pub async fn shorten_url(
  req: web::Json<model::ShortURL>,
) -> Result<HttpResponse, actix_web::Error> {
  let short_url = Box::new(model::ShortURL {
    origin_url: req.origin_url.clone(),
    hashed_url: utils::hash_url(&req.origin_url),
    custom_url: req.custom_url.clone(),
  });

  let url_data = short_url.verify_and_hash().unwrap();

  return Ok(HttpResponse::Ok().json(url_data));
}

pub async fn find_origin_url(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
  let url_param = req.match_info().get("url");
  let short_url = Box::new(model::ShortURL {
    origin_url: "".to_string(),
    hashed_url: url_param.unwrap().to_string(),
    custom_url: url_param.unwrap().to_string(),
  });

  let url_data = short_url.fetch_origin().unwrap();

  return Ok(HttpResponse::Ok().json(url_data));
}

pub async fn migrate_db() -> Result<HttpResponse, actix_web::Error> {
  db::create_table().unwrap();
  return Ok(HttpResponse::Ok().body("Created table shortenurl"));
}
