use crate::shorten::controller::ShortURLController as Controller;
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
    let existing_url = Controller::get_url_by_origin(self.clone())?;

    if existing_url.len() > 0 {
      self.hashed_url = existing_url[0].get(2);
    } else {
      Controller::insert_payload(self.clone())?;
    }

    return Ok(Box::new(self));
  }

  pub fn fetch_origin(mut self) -> Result<Box<ShortURL>, postgres::Error> {
    let existing_url = Controller::get_source_url(self.clone())?;

    match existing_url.len() {
      0 => self.origin_url = "/".to_string(),
      _ => {
        self.origin_url = existing_url.get(1);
        Controller::update_redirection_count(self.clone())?;
      }
    }

    return Ok(Box::new(self));
  }
}
