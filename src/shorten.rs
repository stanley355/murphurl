use actix_web::{ web, Result };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    username: String,
}

/// extract `Info` using serde
pub async fn shorten_url(info: web::Json<Info>) -> Result<String> {
  Ok(format!("Welcome {}!", info.username))
}
