use serde::{Deserialize, Serialize};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug)]
pub struct RequestURL {
  pub origin_url: String,
  pub custom_url: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ResponseURL {
  pub origin_url: String,
  pub hashed_url: String,
  pub custom_url: String,
}

impl ResponseURL {
  pub fn insert_new_url(self, mut client: Box<postgres::Client>) -> Result<(), postgres::Error> {
    let query =
      Box::new("INSERT INTO shortenurl (origin_url, hashed_url, custom_url) VALUES ($1, $2, $3)");
    let result = client.execute(
      *query,
      &[&self.origin_url, &self.hashed_url, &self.custom_url],
    );

    Ok(println!("Affected rows: {:?}", &result))
  }

  // Check if url exist in db, if not insert new one
  pub fn verify_and_hash(
    mut self,
    mut client: Box<postgres::Client>,
  ) -> Result<Box<ResponseURL>, postgres::Error> {
    let query = Box::new("SELECT * FROM shortenurl WHERE origin_url = $1");
    let existing_url = Box::new(client.query(*query, &[&self.origin_url]).unwrap());

    match existing_url.len() {
      0 => self.clone().insert_new_url(client).unwrap(),
      _ => self.hashed_url = existing_url[0].get(2),
    };

    return Ok(Box::new(self));
  }

  pub fn fetch_origin_url(
    mut self,
    mut client: Box<postgres::Client>,
  ) -> Result<Box<ResponseURL>, postgres::Error> {
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

// Hash the origin_url and slice the first to sixth chars as the identifier
pub fn hash_url(url: &String) -> String {
  let mut hasher = DefaultHasher::new();
  url.hash(&mut hasher);
  let num_id = &hasher.finish().to_string();

  return String::from(&num_id[0..6]);
}
