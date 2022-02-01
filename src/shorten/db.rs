use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::env;

use crate::shorten::structs;

fn connect_pg() -> Result<Box<Client>, Error> {
    dotenv().ok();
    let client = Box::new(Client::connect(&env::var("PG_URL").unwrap(), NoTls)?);
    return Ok(client);
}

pub fn create_table() -> Result<(), Error> {
    let mut client = connect_pg().expect("Can't connect to db");
    let query = Box::new(
        "CREATE TABLE IF NOT EXISTS shortenurl (
        id SERIAL PRIMARY kEY,
        origin_url VARCHAR(255) NOT NULL,
        hashed_url VARCHAR(50),
        custom_url VARCHAR(50)
    )",
    );

    client
        .batch_execute(&query)
        .expect("Failed to create table");
    client.close()?;
    return Ok(());
}

pub fn insert_url_data(params: Box<structs::ResponseURL>) -> Result<(), Error> {
    let mut client = connect_pg().expect("Can't connect to db");

    let query =
        Box::new("INSERT INTO shortenurl (origin_url, hashed_url, custom_url) VALUES ($1, $2, $3)");

    let insert_row = client.execute(
        *query,
        &[&params.origin_url, &params.hashed_url, &params.custom_url],
    );

    client.close()?;

    Ok(println!("Affected rows: {:?}", &insert_row))
}

pub fn check_url_data(
    params: Box<structs::ResponseURL>,
) -> Result<Box<structs::ResponseURL>, Error> {
    let mut client = connect_pg().expect("Can't connect to db");

    let query = Box::new("SELECT * FROM shortenurl WHERE origin_url = $1");
    let url_row = Box::new(client.query(*query, &[&params.origin_url]).unwrap());

    client.close()?;

    let mut data = Box::new(structs::ResponseURL {
        origin_url: "".to_string(),
        hashed_url: "".to_string(),
        custom_url: "".to_string(),
    });

    if url_row.len() > 0 {
        data.origin_url = url_row[0].get(1);
        data.hashed_url = url_row[0].get(2);
        data.custom_url = url_row[0].get(3);
        return Ok(data);
    } else {
        return Ok(data);
    }
}
