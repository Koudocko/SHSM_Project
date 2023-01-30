use std::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, Arc},
    thread, error::Error
};
use netstruct::*;
use netstruct::models::NewUser;
use serde_json::{Value, json};

// const SOCKET: &str = "192.168.2.5:7878";
const SOCKET: &str = "127.0.0.1:7878";

fn handle_connection(stream: &mut (TcpStream, bool))-> Result<(), Box<dyn Error>> {
    let request = read_stream(&mut stream.0);
    println!("REQUEST - verified: {}, payload: {request:?}", stream.1);

    let mut header = String::from("GOOD");
    let payload = match request.header.as_str(){
        "CHECK_ACCOUNT" =>{
            if !check_username(serde_json::from_str::<Value>(&request.payload)?)?{
                header = String::from("BAD");
                json!({ "error": "Username already exists! Please change to continue..." }).to_string()
            }
            else{
                String::new()
            }
        }
        "CHECK_CLASS" =>{
            let status = check_course_code(serde_json::from_str::<Value>(&request.payload)?)?;
            if status.0 && !status.1{
                header = String::from("BAD");
                json!({ "error": "Class code already exists! Please change to continue..." }).to_string()
            }
            else if !status.0 && !status.1{
                header = String::from("BAD");
                json!({ "error": "Class code does not exist! Please change to continue..." }).to_string()
            }
            else{
                String::new()
            }
        }
        "CREATE_ACCOUNT" =>{
            if store_in_database(serde_json::from_str::<NewUser>(&request.payload)?).is_err(){
               header = String::from("BAD");
               json!({ "error": "Failed to signup! Please try again..." }).to_string()
            }
            else{
                String::new()
            }
        }
        "GET_ACCOUNT_KEYS" =>{
            if let Some(keys) = get_account_keys(serde_json::from_str::<Value>(&request.payload)?)?{
                keys 
            }
            else{
                header = String::from("BAD");
                json!({ "error": "User does not exist! Please enter a valid username..." }).to_string()
            }
        }
        "VALIDATE_KEY" =>{
            if let Some(verify) = validate_key(serde_json::from_str::<Value>(&request.payload)?)?{
                if !verify{
                    header = String::from("BAD");
                    json!({ "error": "Password is invalid! Please re-enter your password..." }).to_string()
                }
                else{
                    stream.1 = true;
                    String::new() 
                }
            }
            else{
                header = String::from("BAD");
                json!({ "error": "Username does not exist! Please enter a valid username..." }).to_string()
            }
        }
        _ =>{
            String::new()
        }
    };

    write_stream(&mut stream.0, 
        Package{ 
            header,
            payload, 
        }
    ).unwrap();

    Ok(())
}

fn check_connections(streams: Arc<Mutex<Vec<(TcpStream, bool)>>>){
    loop{
        streams.lock().unwrap().retain_mut(|stream|{
            let mut buf = [0u8];
            if let Ok(peeked) = stream.0.peek(&mut buf){
                if peeked != 0{
                    if handle_connection(stream).is_err(){
                        println!("SHUTTING Down Stream");
                        stream.0.shutdown(std::net::Shutdown::Both).unwrap();
                        return false;
                    }

                    return true;
                }

                return true;
            }

            true
        });
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
            streams.lock().unwrap().push((stream, false));
        }
        else{
            println!("Failed to establish connection!");
        }
    }
}
