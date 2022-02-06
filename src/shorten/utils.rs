use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_url(url: &String) -> String {
  // create random number and slice the first to sixth chars
  let mut hasher = DefaultHasher::new();
  url.hash(&mut hasher);
  let num_id = &hasher.finish().to_string();

  return String::from(&num_id[0..6]);
}
