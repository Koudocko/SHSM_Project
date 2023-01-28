use serde::{Serialize, Deserialize};
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream
};
use schema::users::dsl::*;
use diesel::{
    pg::PgConnection,
    prelude::*,
    result::Error
};
use models::*;

pub mod schema;
pub mod models;

#[derive(Clone)]
pub enum Page{
    Certifications, 
    ShsmSelection,
    Events,
    Login,
    Home
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package{
    pub header: String,
    pub payload: String
}

pub fn write_stream(stream: &mut TcpStream, package: Package)-> Result<(), std::io::Error>{
    let mut buf: Vec<u8> = serde_json::to_vec(&package)?;
    buf.push(b'\n');
    stream.write_all(&mut buf)?;

    Ok(())
}

pub fn read_stream(stream: &mut TcpStream)-> Package{
    let mut buf = String::new();

    BufReader::new(stream)
        .read_line(&mut buf)
        .unwrap();

    serde_json::from_str(&buf).unwrap()
}

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres@localhost/SHSM_Project";

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn exists_in_database(pattern: &str)-> bool{
    let connection = &mut establish_connection();

    users.filter(username.eq(pattern)).first::<User>(connection).is_ok()
}

pub fn store_in_database(new_user: NewUser)-> Result<usize, Error>{
    let connection = &mut establish_connection();

    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(connection)
}
