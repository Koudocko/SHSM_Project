use serde::{Serialize, Deserialize};
use serde_json::to_vec;
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream
};

#[derive(Serialize, Deserialize)]
pub struct Account{
    pub username: String,
    pub hash: [[u8; 32]; 2],
    pub salt: [[u8; 32]; 2],
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
    let buf = &mut serde_json::to_vec(&package)?;
    buf.push('\n' as u8);
    stream.write_all(buf)?;

    Ok(())
}

pub fn read_stream(stream: &mut TcpStream)-> Package{
    let buffer: String = BufReader::new(stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    serde_json::from_str(&buffer).unwrap()
}
