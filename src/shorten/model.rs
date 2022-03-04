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
  pub fn new() -> Self {
    Self {
      origin_url: "".to_string(),
      hashed_url: "".to_string(),
      custom_url: "".to_string(),
    }
  }

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BulkShortURL {
  pub url_list: Vec<String>
}

impl BulkShortURL {
  pub fn new(list: Vec<String>) -> Self {
    Self {
      url_list: list
    }
  }

  pub fn collect_and_hash(self) -> Vec<Box<ShortURL>> {
    let mut shorturl_list: Vec<ShortURL> = vec![];
    for url in self.url_list {
      let mut short_url = ShortURL::new();
      short_url.origin_url = url;

      shorturl_list.push(short_url)
    }

    return BulkShortURL::bulk_hash(shorturl_list);
  }

  pub fn bulk_hash(list: Vec<ShortURL>) -> Vec<Box<ShortURL>> {
    let mut new_list: Vec<Box<ShortURL>> = vec![];
    let mut count = 0;

    while count < list.len() {
      let mut short_url = list[count].clone();
      short_url.hashed_url = hash_url(&short_url.origin_url);

      let new_short_url = short_url.verify_and_hash();

      new_list.push(new_short_url.unwrap());

      count += 1;
    }

    return new_list;
  }
}
