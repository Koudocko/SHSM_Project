use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use netstruct::*;

const SOCKET: &str = "192.168.2.5:7878";

fn exists_in_database(entry: &str)-> bool{ false }

fn handle_connection(mut stream: TcpStream) {
    let request = read_stream(&mut stream);
    println!("Request: {:?}", request);

    let response = match request.header.as_str(){
        "CHECK_ACCOUNT" =>{
            if exists_in_database(&request.payload){
                "EXISTS\n\n"
            }
            else{
                "!EXISTS\n\n"
            }
        }
        "CREATE_ACCOUNT" =>{
            ""
        }
        _ =>{
            ""
        }
    }.to_owned();

    write_stream(&mut stream, Package{ header: String::from("GOOD"), payload: response});
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
