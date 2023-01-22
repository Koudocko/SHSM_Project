use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const SOCKET: &str = "192.168.2.241:7878";

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
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
