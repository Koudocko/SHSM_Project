use std::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, Arc},
    thread
};
use netstruct::*;
use netstruct::models::NewUser;

// const SOCKET: &str = "192.168.2.5:7878";
const SOCKET: &str = "127.0.0.1:7878";

fn handle_connection(stream: &mut TcpStream) {
    let request = read_stream(stream);
    println!("{request:?}");

    let mut header = String::from("GOOD");
    let payload = match request.header.as_str(){
        "CHECK_ACCOUNT" =>{
            if exists_in_database(&request.payload){
                header = String::from("BAD");
                "Username already exists! Please change to continue..."
            }
            else{
                ""
            }
        }
        "CREATE_ACCOUNT" =>{
            if store_in_database(serde_json::from_str::<NewUser>(&request.payload).unwrap()).is_err(){
               header = String::from("BAD");
               "Failed to signup! Please try again..."
            }
            else{
                ""
            }
        }
        "GET_ACCOUNT_KEYS" =>{
            "" 
        }
        _ =>{
            ""
        }
    }.to_owned();

    write_stream(stream, 
        Package{ 
            header,
            payload, 
        }
    ).unwrap();
}

fn check_connections(streams: Arc<Mutex<Vec<TcpStream>>>){
    loop{
        for stream in &mut *streams.lock().unwrap(){
            let mut buf = [0u8];
            if let Ok(peeked) = stream.peek(&mut buf){
                if peeked != 0{
                    handle_connection(stream);
                }
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
