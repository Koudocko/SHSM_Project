use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use netstruct::*;

const SOCKET: &str = "192.168.2.5:7878";

fn exists_in_database(entry: &Entry)-> bool{ true }

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", request);

    let response = match request[0].as_str(){
        "EXISTS" =>{
            let entry: Entry = serde_json::from_str(&request[1]).unwrap();
            
            if exists_in_database(&entry){
                "GOOD\n\n"
            }
            else{
                "BAD\n\n"
            }
        }
        _ =>{
            ""
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind(SOCKET).unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream{
            println!("Connection established!");
            handle_connection(stream);
        }
        else{
            println!("Failed to establish connection!");
        }
    }
}
