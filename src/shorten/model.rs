use crate::shorten::db::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShortURL {
  pub origin_url: String,
  pub hashed_url: String,
  pub custom_url: String,
}

impl ShortURL {
  // Check if url exist in db, if not insert new one
  pub fn verify_and_hash(mut self) -> Result<Box<ShortURL>, postgres::Error> {
    let existing_url = get_url_by_origin(self.clone())?;

    if existing_url.len() > 0 {
      self.hashed_url = existing_url.get(2);
    } else {
      insert_payload(self.clone())?;
    }

    return Ok(Box::new(self));
  }

  pub fn fetch_origin_url(mut self) -> Result<Box<ShortURL>, postgres::Error> {
    let existing_url = get_source_url(self.clone())?;

    match existing_url.len() {
      0 => self.origin_url = "/".to_string(),
      _ => {
        update_redirection_count(self.clone())?;
        self.origin_url = existing_url.get(1);
      }
    }

    return Ok(Box::new(self));
  }
}
