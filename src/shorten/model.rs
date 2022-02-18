use crate::shorten::db::{insert_payload, get_url_by_origin};
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
      insert_payload(self.clone());
    } else {
      self.hashed_url = existing_url.get(2);
    }

    return Ok(Box::new(self));
  }

  pub fn fetch_origin_url(
    mut self,
    mut client: Box<postgres::Client>,
  ) -> Result<Box<ShortURL>, postgres::Error> {
    let query = Box::new("SELECT * FROM shortenurl WHERE hashed_url = $1 OR custom_url = $2");
    let result = Box::new(
      client
        .query(*query, &[&self.hashed_url, &self.custom_url])
        .unwrap(),
    );

    match result.len() {
      0 => self.origin_url = "/".to_string(),
      _ => {
        let origin_url: String = result[0].get(1);

        // Redirection count logic
        let update_query = Box::new(
          "UPDATE shortenurl SET redirection_count = redirection_count + 1 WHERE origin_url = $1 ",
        );
        let update_result = Box::new(
          client
            .execute(&update_query.to_string(), &[&origin_url])
            .unwrap(),
        );
        println!("Affected rows: {:?}", &update_result);

        // Response
        self.origin_url = origin_url;
      }
    }

    return Ok(Box::new(self));
  }
}
