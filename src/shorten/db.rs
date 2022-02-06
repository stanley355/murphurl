use dotenv::dotenv;
use std::env;

use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres::{Client, Error};
use postgres_openssl::MakeTlsConnector;

use crate::shorten::structs;

fn connect_pg() -> Result<Box<Client>, Error> {
    dotenv().ok();

    // Create Ssl postgres connector without verification as required to connect to Heroku.
    let mut ssl = SslConnector::builder(SslMethod::tls()).unwrap();
    ssl.set_verify(SslVerifyMode::NONE);
    let tls = MakeTlsConnector::new(ssl.build());

    let client = Box::new(Client::connect(&env::var("PG_URL").unwrap(), tls)?);
    return Ok(client);
}

pub fn insert_new_url(params: Box<structs::ResponseURL>) -> Result<(), Error> {
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

pub fn check_existing_url(
    params: Box<structs::ResponseURL>,
) -> Result<Box<structs::ResponseURL>, Error> {
    let mut client = connect_pg().expect("Can't connect to db");
    let query: Box<_>;
    let url_row: Box<_>;

    if params.origin_url == "" {
        query = Box::new("SELECT * FROM shortenurl WHERE hashed_url = $1 OR custom_url = $2");
        url_row = Box::new(
            client
                .query(*query, &[&params.hashed_url, &params.custom_url])
                .unwrap(),
        );
    } else {
        query = Box::new("SELECT * FROM shortenurl WHERE origin_url = $1");
        url_row = Box::new(client.query(*query, &[&params.origin_url]).unwrap());
    }

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
