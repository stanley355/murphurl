use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestURL {
  pub origin_url: String,
  pub custom_url: String,
  pub expired_date: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ResponseURL {
  pub origin_url: String,
  pub hashed_url: String,
  pub custom_url: String,
  pub expired_date: String,
}
