use std::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, Arc},
    thread, error::Error
};
use netstruct::*;
use netstruct::models::{NewUser, User, NewEvent};
use serde_json::{Value, json};

// const SOCKET: &str = "192.168.2.5:7878";
const SOCKET: &str = "127.0.0.1:7878";

fn handle_connection(stream: &mut (TcpStream, Option<User>))-> Result<(), Box<dyn Error>> {
    stream.0.set_nonblocking(false).unwrap();
    let request = read_stream(&mut stream.0);
    println!("INCOMING REQUEST\nVerified: {:?}\nHeader: {}\nPayload: {:?}", stream.1.is_some(), request.header, request.payload);

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
                if !verify.1{
                    header = String::from("BAD");
                    json!({ "error": "Password is invalid! Please re-enter your password..." }).to_string()
                }
                else{
                    stream.1 = Some(verify.0.clone());
                    json!({ "is_teacher": verify.0.teacher }).to_string()
                }
            }
            else{
                header = String::from("BAD");
                json!({ "error": "Username does not exist! Please enter a valid username..." }).to_string()
            }
        }
        "GET_ANNOUNCEMENTS" =>{
            if let Some(user) = &stream.1{
                json!({ "announcements": get_announcements(&user.code) }).to_string()
            }
            else{
               return Err(Box::new(PlainError::new()))
            }
        }
        "ADD_ANNOUNCEMENT" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    if add_announcement(serde_json::from_str::<Value>(&request.payload)?, user.id)?{
                        String::new()
                    }
                    else{
                        header = String::from("BAD");
                        json!({ "error": "Announcement title already exists! Please change to continue..." }).to_string()
                    }
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "GET_USER_EVENTS" =>{
            if let Some(user) = &stream.1{
                json!({ "events": get_user_events(&user.username) }).to_string()
            }
            else{
               return Err(Box::new(PlainError::new()))
            }
        }
        "GET_USER_EVENTS_CL" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    if let Some(username) = unpack(&request.payload, "username").as_str(){
                        json!({ "events": get_user_events(username) }).to_string()
                    }
                    else{
                       return Err(Box::new(PlainError::new()))
                    }
                }
                else{
                   return Err(Box::new(PlainError::new()))
                }
            }
            else{
               return Err(Box::new(PlainError::new()))
            }
        }
        "GET_SHSM_EVENTS" =>{
            if let Some(user) = &stream.1{
                json!({ "events": get_shsm_events(&user.code) }).to_string()
            }
            else{
               return Err(Box::new(PlainError::new()))
            }
        }
        "ADD_SHSM_EVENT" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    if add_shsm_event(serde_json::from_str::<NewEvent>(&request.payload)?, user.id){
                        String::new()
                    }
                    else{
                        header = String::from("BAD");
                        json!({ "error": "Event title already exists! Please change to continue..." }).to_string()
                    }
                }
                else{
                    return Err(Box::new(PlainError::new()));
                }
            }
            else{
                return Err(Box::new(PlainError::new()));
            }
        }
        "ADD_USER_EVENT" =>{
            if let Some(user) = &stream.1{
                add_user_event(serde_json::from_str::<Value>(&request.payload)?, user.id, &user.code)?;
                String::new()
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "REMOVE_USER_EVENT" =>{
            if let Some(user) = &stream.1{
                remove_user_event(serde_json::from_str::<Value>(&request.payload)?, user.id)?;
                String::new()
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "CERTIFY_USER" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    certify_user(serde_json::from_str::<Value>(&request.payload)?, &user.code)?;
                    String::new()
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "GET_EVENT_USERS" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    json!({ "users": get_event_users(serde_json::from_str::<Value>(&request.payload)?, &user.code)? }).to_string()
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "REMOVE_USER" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    remove_user(serde_json::from_str::<Value>(&request.payload)?, &user.code)?;
                    String::new()
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "UPDATE_USER" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    if update_user(serde_json::from_str::<Value>(&request.payload)?, &user.code)?{
                        String::new()
                    }
                    else{
                        header = String::from("BAD");
                        json!({ "error": "Username already exists! Please change to continue..." }).to_string()
                    }
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "UPDATE_EVENT" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    if update_event(serde_json::from_str::<Value>(&request.payload)?, user.id, &user.code)?{
                        String::new()
                    }
                    else{
                        header = String::from("BAD");
                        json!({ "error": "Event title already exists! Please change to continue..." }).to_string()
                    }
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "REMOVE_EVENT" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    remove_event(serde_json::from_str::<Value>(&request.payload)?, &user.code)?;
                    String::new()
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "REMOVE_ANNOUNCEMENT" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    remove_announcement(serde_json::from_str::<Value>(&request.payload)?, user.id)?;
                    String::new()
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
            }
        }
        "GET_CLASS_LIST" =>{
            if let Some(user) = &stream.1{
                if user.teacher{
                    json!({ "class_list": get_class_list(&user.code)? }).to_string()
                }
                else{
                   return Err(Box::new(PlainError::new()));
                }
            }
            else{
               return Err(Box::new(PlainError::new()));
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

fn check_connections(streams: Arc<Mutex<Vec<(TcpStream, Option<User>)>>>){
    loop{
        streams.lock().unwrap().retain_mut(|stream|{
            let mut buf = [0u8];
            stream.0.set_nonblocking(true).unwrap();
            if let Ok(peeked) = stream.0.peek(&mut buf){
                if peeked != 0{
                    if handle_connection(stream).is_err(){
                        println!("SHUTTING Down Stream");
                        stream.0.shutdown(std::net::Shutdown::Both).unwrap();
                        return false;
                    }
                }
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
            streams.lock().unwrap().push((stream, None));
            println!("Connection added!");
        }
        else{
            println!("Failed to establish connection!");
        }
    }
}
