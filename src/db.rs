use postgres::{Client, NoTls, Error};
use dotenv::dotenv;
use std::env;

pub fn connect_pg() -> Result<(), Error> {
    dotenv().ok();
    let mut client = Client::connect(&env::var("PG_URL").unwrap(), NoTls)?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ")?;

    Ok(())

}
