use serde::{Serialize, Deserialize};
use diesel::Queryable;
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream
};

#[derive(Serialize, Deserialize)]
pub struct Account{
    pub mode: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub enum Page{
    Certifications, 
    ShsmSelection,
    Events,
    Login,
    Home
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Entry{
    pub key: String,
    pub val: String
}

pub fn package_stream(stream: &mut TcpStream)-> Vec<String>{
    BufReader::new(stream)
        .lines()
        .map(|result|{
            if let Ok(result) = result{
                result
            }
            else{
                String::new()
            }
        })
        .take_while(|line| !line.is_empty())
        .collect()
}
