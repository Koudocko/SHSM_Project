use serde::{Serialize, Deserialize};
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream
};

#[derive(Serialize, Deserialize)]
pub struct Account{
    pub username: String,
    pub teacher: bool,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}

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
