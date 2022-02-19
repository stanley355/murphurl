use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres::{Client, Error};
use postgres_openssl::MakeTlsConnector;

use dotenv::dotenv;
use std::env;

pub fn pg_client() -> Result<Box<Client>, Error> {
  dotenv().ok();
  let pg_url = &env::var("PG_URL").expect("Can't find PG URL");

  // Create Ssl postgres connector without verification as required to connect to Heroku.
  let mut ssl = SslConnector::builder(SslMethod::tls()).unwrap();
  ssl.set_verify(SslVerifyMode::NONE);
  let tls = MakeTlsConnector::new(ssl.build());

  let client = Box::new(Client::connect(&pg_url, tls)?);
  return Ok(client);
}

pub fn create_table() -> Result<(), Error> {
  let mut client = pg_client().unwrap();
  let query = Box::new(
    "CREATE TABLE IF NOT EXISTS shortenurl (
      id SERIAL PRIMARY kEY,
      origin_url VARCHAR(255) NOT NULL,
      hashed_url VARCHAR(50),
      custom_url VARCHAR(50),
      redirection_count INT DEFAULT 0
  );",
  );

  client.batch_execute(*query).expect("Fail to create DB");
  client.close().unwrap();

  return Ok(());
}
