use serde::{Serialize, Deserialize};
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream, error::Error
};
use schema::users::dsl::*;
use diesel::{
    pg::PgConnection,
    prelude::*,
};
use models::*;
use serde_json::{json, Value};
use std::fmt;

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

#[derive(Debug, Clone)]
struct PlainError;
impl PlainError{
    fn new()-> PlainError{ PlainError }
}
impl fmt::Display for PlainError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
impl Error for PlainError{}

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

pub fn exists_in_database(pattern: Value)-> Result<bool, Box<dyn Error>>{
    let connection = &mut establish_connection();

    let pattern = pattern["username"].as_str();
    if pattern.is_none(){
       Err(Box::new(PlainError::new())) 
    }
    else{
        Ok(users.filter(username.eq(pattern.unwrap())).first::<User>(connection).is_ok())
    }
}

pub fn store_in_database(new_user: NewUser)-> Result<usize, Box<dyn Error>>{
    let connection = &mut establish_connection();

    Ok(diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(connection)?)
}

pub fn get_account_keys(pattern: Value)-> Result<Option<String>, Box<dyn Error>>{
    let connection = &mut establish_connection();

    if let Some(pattern) = pattern["username"].as_str(){
        if let Ok(user) = users.filter(username.eq(pattern)).first::<User>(connection){
            Ok(Some(json!({ "hash": user.hash, "salt": user.salt }).to_string()))
        }
        else{
            Ok(None)
        }
    }
    else{
       Err(Box::new(PlainError::new()))
    }
}
