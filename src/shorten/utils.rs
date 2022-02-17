use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Hash the origin_url and slice the first to sixth chars as the identifier
pub fn hash_url(url: &String) -> String {
  let mut hasher = DefaultHasher::new();
  url.hash(&mut hasher);
  let hashed_url = &hasher.finish().to_string();

  return String::from(&hashed_url[0..6]);
}
