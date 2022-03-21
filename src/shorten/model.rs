use crate::shorten::controller::ShortURLController as Controller;
use crate::shorten::utils::hash_url;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShortURL {
  pub origin_url: String,
  pub hashed_url: String,
  pub custom_url: String,
}

impl ShortURL {
  // Check if url exist in db, if not insert new one
  pub fn verify_and_hash(mut self) -> Result<ShortURL, postgres::Error> {
    let existing_url = Controller::get_url_by_origin(self.clone())?;

    if existing_url.len() > 0 {
      self.hashed_url = existing_url[0].get(2);
    } else {
      Controller::insert_payload(self.clone())?;
    }

    return Ok(self);
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BulkShortURL {
  pub shorturl_list: Vec<ShortURL>
}

impl BulkShortURL {
  pub fn bulk_hash(self) -> Vec<ShortURL> {
    let old_list = self.shorturl_list;
    let mut new_list: Vec<ShortURL> = vec![];
    let mut count = 0;

    while count < old_list.len() {
      let mut short_url = old_list[count].clone();
      short_url.hashed_url = hash_url(&short_url.origin_url);
      let new_short_url = short_url.verify_and_hash();

      new_list.push(new_short_url.unwrap());

      count += 1;
    }

    return new_list;
  }
}
