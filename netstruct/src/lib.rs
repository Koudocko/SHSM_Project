use serde::{Serialize, Deserialize};
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream, error::Error
};
use schema::{users::dsl::*, announcements::description};
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
pub struct PlainError;
impl PlainError{
    pub fn new()-> PlainError{ PlainError }
}
impl fmt::Display for PlainError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
impl Error for PlainError{}

pub fn unpack(payload: &str, field: &str)-> Value{
    serde_json::from_str::<Value>(payload).unwrap()[field].clone()
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

    println!("BUF: {buf}");
    serde_json::from_str(&buf).unwrap()
}

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres@localhost/SHSM_Project";

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn check_username(payload: Value)-> Result<bool, Box<dyn Error>>{
    let connection = &mut establish_connection();

    if let Some(payload) = payload["username"].as_str(){
        Ok(users.filter(username.eq(payload)).first::<User>(connection).is_err())
    }
    else{
        Err(Box::new(PlainError::new())) 
    }
}

pub fn check_course_code(payload: Value)-> Result<(bool, bool), Box<dyn Error>>{
    let connection = &mut establish_connection();

    if let Some(user_username) = payload["course_code"].as_str(){
        let exists = users.filter(teacher.eq(true))
            .filter(code.eq(user_username))
            .first::<User>(connection)
            .is_ok();

        if let Some(user_is_teacher) = payload["is_teacher"].as_bool(){
            if user_is_teacher{
                return Ok((user_is_teacher, !exists));
            }
            else{
                return Ok((user_is_teacher, exists));
            }
        }
    }

    Err(Box::new(PlainError::new())) 
}

pub fn store_in_database(new_user: NewUser)-> Result<usize, Box<dyn Error>>{
    let connection = &mut establish_connection();

    Ok(diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(connection)?)
}

pub fn get_account_keys(payload: Value)-> Result<Option<String>, Box<dyn Error>>{
    let connection = &mut establish_connection();

    if let Some(payload) = payload["username"].as_str(){
        if let Ok(user) = users.filter(username.eq(payload)).first::<User>(connection){
            Ok(Some(json!({ "salt": user.salt }).to_string()))
        }
        else{
            Ok(None)
        }
    }
    else{
       Err(Box::new(PlainError::new()))
    }
}

pub fn validate_key(payload: Value)-> Result<Option<(User, bool)>, Box<dyn Error>>{
    let connection = &mut establish_connection();

    if let Some(user_hash) = payload["hash"].as_array(){
        let user_hash = user_hash.into_iter().map(|byte|{
            if let Some(byte) = byte.as_u64(){
                if let Ok(byte) = u8::try_from(byte){
                    return byte
                }
            }

            0
        }).collect::<Vec<u8>>();

        if let Some(user_username) = payload["username"].as_str(){
            if let Ok(user) = users.filter(username.eq(user_username)).first::<User>(connection){
                let mut idx = 0;
                let verified = !user_hash.iter().any(|byte|{
                    let check = *byte != user.hash[idx];
                    idx += 1;
                    check
                });

                return Ok(Some((user, verified)));
            }
            else{
                return Ok(None);
            }
        }
    }

   Err(Box::new(PlainError::new()))
}

pub fn get_announcements(class_code: &str)-> Vec<Announcement>{
    let connection = &mut establish_connection();

    let course = users.filter(teacher.eq(true))
        .filter(code.eq(class_code))
        .first::<User>(connection)
        .unwrap();

    Announcement::belonging_to(&course)
        .load::<Announcement>(connection)
        .expect("Error loading announcements")
}

pub fn add_announcement(payload: Value, user_id: i32)-> Result<(), Box<dyn Error>>{
    let connection = &mut establish_connection();

    if let Some(announcement_title) = payload["title"].as_str(){
        if let Some(announcement_description) = payload["description"].as_str(){
            let new_announcement = NewAnnouncement{
                title: announcement_title.to_owned(),
                description: announcement_description.to_owned(),
                user_id
            };

            diesel::insert_into(schema::announcements::table)
                .values(&new_announcement)
                .execute(connection)
                .expect("Failed to insert announcment!");
            }

        return Ok(());
    }

    Err(Box::new(PlainError::new()))
}

pub fn get_certifications(name: &str)-> Vec<Event>{
    let connection = &mut establish_connection();

    let user = users.filter(username.eq(name))
        .first::<User>(connection)
        .unwrap();

    Event::belonging_to(&user)
        .load::<Event>(connection)
        .expect("Error loading certifications!")
        .into_iter()
        .filter(|event| event.certification)
        .collect()
}
