use std::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, Arc},
    thread
};

use netstruct::*;
use schema::users::dsl::*;
use diesel::prelude::*;
use models::*;
use db_tools::*;

mod models;
mod schema;
mod db_tools;

// const SOCKET: &str = "192.168.2.5:7878";
const SOCKET: &str = "127.0.0.1:7878";

fn exists_in_database(_: &str)-> bool{ false }
fn store_in_database(_: Account){}

fn handle_connection(stream: &mut TcpStream) {
    let request = read_stream(stream);
    println!("{request:?}");

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
            store_in_database(serde_json::from_str(&request.payload).unwrap());
            ""
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
            header: String::from("GOOD"), 
            payload: response
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
    // let listener = TcpListener::bind(SOCKET).unwrap();
    // let streams = Arc::new(Mutex::new(Vec::new()));

    // let handle = Arc::clone(&streams);
    // thread::spawn(||{
    //     check_connections(handle);
    // });

    // for stream in listener.incoming(){
    //     if let Ok(stream) = stream{
    //         println!("Connection established!");
    //         streams.lock().unwrap().push(stream);
    //     }
    //     else{
    //         println!("Failed to establish connection!");
    //     }
    // }
    
    let connection = &mut establish_connection();
    let new_user = NewUser{ 
        username: String::from("joe"),
        hash: Vec::new(),
        salt: Vec::new(),
        teacher: true,
        code: None
    };

    // diesel::insert_into(schema::users::table)
    //     .values(&new_user)
    //     .execute(connection)
    //     .expect("Failed to insert user!");

    // diesel::update(users.find(1))
    //     .set(username.eq("JOE BIDEN"))
    //     .execute(connection)
    //     .expect("Failed to update user!");

    

    // let results = users
    //     .load::<User>(connection)
    //     .expect("Error loading posts");


    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user.id);
        println!("{:?}", user.username);
        println!("---------------------");
    }
}
