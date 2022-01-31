use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::env;

use crate::shorten::structs;

fn connect_pg() -> Result<Client, Error> {
    dotenv().ok();
    return Ok(Client::connect(&env::var("PG_URL").unwrap(), NoTls)?);
}

pub fn create_table() -> Result<(), Error> {
    let mut client = connect_pg().expect("Can't connect to db");
    let query = "CREATE TABLE IF NOT EXISTS shortenurl (
        id SERIAL PRIMARY kEY,
        origin_url VARCHAR(255) NOT NULL,
        hashed_url VARCHAR(50),
        custom_url VARCHAR(50),
    )";

    client.batch_execute(&query)?;
    client.close()?;
    return Ok(println!("Shortenurl table is working"));
}

pub fn insert_url_data(params: structs::ResponseURL) -> Result<(), Error> {
    let mut client = connect_pg().expect("Can't connect to db");

    let query = "INSERT INTO shortenurl (origin_url, hashed_url, custom_url, expired_date)
    VALUES ($1, $2, $3, $4) RETURNING id";

    let insert_row = client.execute(
        query,
        &[
            &params.origin_url,
            &params.hashed_url,
            &params.custom_url,
            &params.expired_date,
        ],
    ).expect("Fail to execute shortenurl INSERT query");

    client.close()?;

    Ok(println!("Affected rows: {:?}", insert_row))
}
