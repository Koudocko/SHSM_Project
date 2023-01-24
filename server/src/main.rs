use std::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, Arc},
    thread
};

use netstruct::*;

// const SOCKET: &str = "192.168.2.5:7878";
const SOCKET: &str = "127.0.0.1:7878";

fn exists_in_database(_: &str)-> bool{ false }

fn handle_connection(stream: &mut TcpStream) {
    let request = read_stream(stream);

    let response = match request.header.as_str(){
        "CHECK_ACCOUNT" =>{
            if exists_in_database(&request.payload){
                "EXISTS"
            }
            else{
                "!EXISTS"
            }
        }
        "CREATE_ACCOUNT" =>{
            ""
        }
        _ =>{
            ""
        }
    }.to_owned();

    write_stream(stream, 
        Package{ 
            header: String::from("GOOD"), 
            payload: response
        }
    ).unwrap();
}

fn check_connections(streams: Arc<Mutex<Vec<TcpStream>>>){
    loop{
        for stream in &mut *streams.lock().unwrap(){
            let mut buf = [0u8];
            if stream.peek(&mut buf).unwrap() != 0{
                handle_connection(stream);
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind(SOCKET).unwrap();
    let streams = Arc::new(Mutex::new(Vec::new()));

    let handle = Arc::clone(&streams);
    thread::spawn(||{
        check_connections(handle);
    });

    for stream in listener.incoming(){
        if let Ok(stream) = stream{
            println!("Connection established!");
            streams.lock().unwrap().push(stream);
        }
        else{
            println!("Failed to establish connection!");
        }
    }
}
