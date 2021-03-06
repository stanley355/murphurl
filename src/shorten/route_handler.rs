use actix_web::{web, Error, HttpRequest, HttpResponse, Result};

use crate::shorten::{db, model};

pub async fn migrate_db() -> Result<HttpResponse, Error> {
  db::create_shortenurl_table().unwrap();
  return Ok(HttpResponse::Ok().body("Created table shortenurl"));
}

// Create shortened url
pub async fn shorten_url(req: web::Json<model::ShortURL>) -> Result<HttpResponse, Error> {
  let short_url = model::ShortURL {
    origin_url: req.origin_url.clone(),
    hashed_url: "".to_string(),
    custom_url: req.custom_url.clone(),
  };

  let url_data = short_url.verify_and_hash().unwrap();

  return Ok(HttpResponse::Ok().json(url_data));
}

pub async fn find_redirect_url(req: HttpRequest) -> Result<HttpResponse, Error> {
  let url_param = req.match_info().get("url");
  let short_url = model::ShortURL {
    origin_url: "".to_string(),
    hashed_url: url_param.unwrap().to_string(),
    custom_url: url_param.unwrap().to_string(),
  };

  let url_data = short_url.get_origin_url().unwrap();

  return Ok(HttpResponse::Ok().json(url_data));
}

pub async fn bulk_upload(req: web::Json<model::BulkShortURL>) -> Result<HttpResponse, Error> {
  let bulk_shorturl = model::BulkShortURL {
    shorturl_list: req.shorturl_list.clone(),
  };
  let short_url_list = bulk_shorturl.bulk_hash();
  return Ok(HttpResponse::Ok().json(short_url_list));
}
