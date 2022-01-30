use postgres::{Client, NoTls, Error};

pub fn connect_pg() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:password@localhost/postgres", NoTls)?;
    
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
