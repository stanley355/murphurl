use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres::{Client, Error, Row};
use postgres_openssl::MakeTlsConnector;

use dotenv::dotenv;
use std::env;

use crate::shorten::model;

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

pub fn insert_payload(data: model::ShortURL) -> Result<u64, postgres::Error> {
  let mut client = pg_client().unwrap();
  let query =
    Box::new("INSERT INTO shortenurl (origin_url, hashed_url, custom_url) VALUES ($1, $2, $3)");

  let result = client.execute(
    *query,
    &[&data.origin_url, &data.hashed_url, &data.custom_url],
  )?;

  client.close()?;

  println!("Affected rows: {:?}", result);
  return Ok(result);
}

pub fn get_url_by_origin(data: model::ShortURL) -> Result<Vec<Row>, Error> {
  let mut client = pg_client().unwrap();
  let query = Box::new("SELECT * FROM shortenurl WHERE origin_url = $1");
  let result = client.query(*query, &[&data.origin_url])?;
  client.close()?;

  return Ok(result);
}

pub fn get_source_url(data: model::ShortURL) -> Result<Row, Error> {
  let mut client = pg_client().unwrap();
  let query = Box::new("SELECT * FROM shortenurl WHERE hashed_url = $1 OR custom_url = $2");
  let result = client.query_one(*query, &[&data.hashed_url, &data.custom_url])?;
  client.close()?;

  return Ok(result);
}

pub fn update_redirection_count(data: model::ShortURL) -> Result<u64, Error> {
  let mut client = pg_client().unwrap();
  let query = Box::new(
    "UPDATE shortenurl SET redirection_count = redirection_count + 1 WHERE origin_url = $1",
  );
  let result = client.execute(*query, &[&data.origin_url])?;
  client.close()?;

  println!("Affected rows: {:?}", result);
  return Ok(result);
}

pub fn create_db() -> Result<(), Error> {
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

  return Ok(println!("Created table shortenurl"));
}
