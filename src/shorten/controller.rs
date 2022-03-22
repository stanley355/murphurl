use crate::shorten::{db, model};
use postgres::{Error, Row};

pub struct ShortURLController;

impl ShortURLController {
  pub fn insert_url_data(data: model::ShortURL) -> Result<u64, Error> {
    let mut client = db::pg_client()?;
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

  pub fn get_hashed_url(data: model::ShortURL) -> Result<Vec<Row>, Error> {
    let mut client = db::pg_client()?;
    let query = Box::new("SELECT * FROM shortenurl WHERE origin_url = $1");
    let result = client.query(*query, &[&data.origin_url])?;
    client.close()?;

    return Ok(result);
  }

  pub fn get_origin_url(data: model::ShortURL) -> Result<Row, Error> {
    let mut client = db::pg_client()?;
    let query = Box::new("SELECT * FROM shortenurl WHERE hashed_url = $1 OR custom_url = $2");
    let result = client.query_one(*query, &[&data.hashed_url, &data.custom_url])?;
    client.close()?;

    return Ok(result);
  }

  pub fn update_redirection_count(data: model::ShortURL) -> Result<u64, Error> {
    let mut client = db::pg_client()?;
    let query = Box::new(
      "UPDATE shortenurl SET redirection_count = redirection_count + 1 WHERE origin_url = $1",
    );
    let result = client.execute(*query, &[&data.origin_url])?;
    client.close()?;

    println!("Affected rows: {:?}", result);
    return Ok(result);
  }
}
