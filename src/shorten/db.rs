use postgres::{Client, NoTls, Error};
use dotenv::dotenv;
use std::env;

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
        expired_date TIMESTAMP,
        created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP 
    )";

    client.batch_execute(query).expect("Fail to create shortenurl Table");
    return Ok(println!("Shortenurl table is working"));
}
