#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::{
    net::TcpStream,
    sync::Mutex,
};
use netstruct::*;
use netstruct::models::{NewUser, Announcement, Event, NewEvent};
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use once_cell::sync::Lazy;
use tauri::{
    api::dialog::MessageDialogBuilder,
    State,
    Window,
    Manager
};
use serde_json::json;

static mut CURRENT_PAGE: String = String::new();
// const SOCKET: &str = "als-kou.ddns.net:7878";
const SOCKET: &str = "127.0.0.1:7878";
static STREAM: Lazy<Mutex<TcpStream>> = Lazy::new(||{
    Mutex::new(TcpStream::connect(SOCKET).unwrap())
});
static mut IS_TEACHER: bool = false; 

struct WindowHandle(Mutex<Window>);

#[tauri::command]
fn modify_user_event(title: String, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("GET_USER_EVENTS"), 
            payload: String::new()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());

    if let Some(events) = unpack(&response.payload, "events").as_array(){
        if events.into_iter().any(|event| serde_json::from_value::<Event>(event.clone()).unwrap().title == title){
            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("REMOVE_USER_EVENT"), 
                    payload: json!({ "title": title }).to_string()
                }
            ).unwrap();
        }
        else{
            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("ADD_USER_EVENT"), 
                    payload: json!({ "title": title }).to_string()
                }
            ).unwrap();
        }
    }
    else{
        write_stream(&mut *STREAM.lock().unwrap(), 
            Package { 
                header: String::from("ADD_USER_EVENT"), 
                payload: json!({ "title": title }).to_string()
            }
        ).unwrap();
    }

    read_stream(&mut *STREAM.lock().unwrap());
    sync_elements(String::from("EVENTS"), window);
}

#[tauri::command]
fn remove_event(title: String, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("REMOVE_EVENT"), 
            payload: json!({ "title": title }).to_string()
        }
    ).unwrap();

    read_stream(&mut *STREAM.lock().unwrap());
    sync_elements(String::from("EVENTS"), window);
}

#[tauri::command]
fn get_event_users(title: String, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("GET_EVENT_USERS"), 
            payload: json!({ "title": title }).to_string()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());

    window.0.lock().unwrap()
        .eval("document.getElementById('students-container').innerHTML = '';")
        .unwrap();

    if let Some(users) = unpack(&response.payload, "users").as_array(){
        for user in users{
            let user: String = serde_json::from_value(user.clone()).unwrap();

            window.0.lock().unwrap()
                .eval(&format!("document.getElementById('students-container').innerHTML += '<p>{}</p>';", user))
                .unwrap();
        }
    }
}

#[tauri::command]
fn update_event(title: String, new_title: String, new_description: String, new_date: String, new_certification: bool, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("UPDATE_EVENT"), 
            payload: json!({ "title": title, "new_title": new_title, "new_description": new_description, "new_date": new_date, "new_certification": new_certification }).to_string()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());
    if response.header == "GOOD"{
        sync_elements(String::from("EVENTS"), window);
    }
    else{
        MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
           .show(|_|{});
    }
}

#[tauri::command]
fn add_shsm_event(title: String, description: String, date: String, certification: bool, window: State<WindowHandle>){
    let new_event = NewEvent{
        title,
        description,
        date,
        certification,
        completed: false,
        user_id: 0
    };

    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("ADD_SHSM_EVENT"), 
            payload: serde_json::to_string(&new_event).unwrap()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());
    if response.header == "GOOD"{
        sync_elements(String::from("EVENTS"), window);
    }
    else{
        MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
           .show(|_|{});
    }
}

#[tauri::command]
fn certify_user(username: String, certification_name: String, checked: bool, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("CERTIFY_USER"), 
            payload: json!({ "username": username, "title": certification_name, "completed": checked }).to_string()
        }
    ).unwrap();

    read_stream(&mut *STREAM.lock().unwrap());
    sync_elements(String::from("CLASSLIST"), window);
}

#[tauri::command]
fn get_user_events(username: String, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("GET_USER_EVENTS_CL"), 
            payload: json!({ "username": username }).to_string()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());

    window.0.lock().unwrap()
        .eval("document.getElementById('certifications-container').innerHTML = '';")
        .unwrap();

    if let Some(events) = unpack(&response.payload, "events").as_array(){
        for event in events{
            let event: Event = serde_json::from_value(event.clone()).unwrap();

            let checked = if event.completed
                {"checked"}
                else
                {""};
            if event.certification{
                window.0.lock().unwrap()
                    .eval(&format!("
                        document.getElementById('certifications-container').innerHTML += `
                          <span>{} <input class='check' type='checkbox' name='{}' {checked}></span>`;
                    ", event.title, event.title))
                    .unwrap();
            }
        }
    }
}

#[tauri::command]
fn remove_announcement(title: String, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("REMOVE_ANNOUNCEMENT"), 
            payload: json!({ "title": title }).to_string()
        }
    ).unwrap();

    read_stream(&mut *STREAM.lock().unwrap());
    sync_elements(String::from("ANNOUNCEMENTS"), window);
}

#[tauri::command]
fn update_user(username: String, new_username: String, new_password: String, window: State<WindowHandle>){
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt_key = [0u8; CREDENTIAL_LEN];
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    if !new_password.is_empty(){
        rng.fill(&mut salt_key).unwrap();

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt_key,
            new_password.as_bytes(),
            &mut pbkdf2_hash,
        );
    }

    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("UPDATE_USER"), 
            payload: json!({ "username": username, "new_username": new_username, "new_hash": pbkdf2_hash.to_vec(), "new_salt": salt_key.to_vec() }).to_string()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());
    if response.header == "GOOD"{
        sync_elements(String::from("CLASSLIST"), window);
    }
    else{
        MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
           .show(|_|{});
    }
}

#[tauri::command]
fn remove_user(username: String, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("REMOVE_USER"), 
            payload: json!({ "username": username }).to_string()
        }
    ).unwrap();

    read_stream(&mut *STREAM.lock().unwrap());
    sync_elements(String::from("CLASSLIST"), window);
}

#[tauri::command]
fn add_event(title: String, description: String, date: String, certification: bool, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("ADD_SHSM_EVENT"), 
            payload: json!({ "title": title, "description": description, "date": date, "certification": certification, "completed": false }).to_string()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());

    if response.header == "GOOD"{
        sync_elements(String::from("EVENTS"), window);
    }
    else{
        MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
           .show(|_|{});
    }
}

#[tauri::command]
fn add_announcement(title: String, description: String, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("ADD_ANNOUNCEMENT"), 
            payload: json!({ "title": title, "description": description }).to_string()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());

    if response.header == "GOOD"{
        sync_elements(String::from("ANNOUNCEMENTS"), window);
    }
    else{
        MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
           .show(|_|{});
    }
}

#[tauri::command]
fn sync_elements(page_name: String, window: State<WindowHandle>){
    unsafe{ CURRENT_PAGE = page_name.to_owned(); }

    println!("ON PAGE: {}", page_name);
    match page_name.as_str(){
        "CERTIFICATIONS" =>{
            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("GET_USER_EVENTS"), 
                    payload: String::new()
                }
            ).unwrap();

            let response = read_stream(&mut *STREAM.lock().unwrap());

            window.0.lock().unwrap()
                .eval("document.getElementsByClassName('card-container')[0].innerHTML = '';")
                .unwrap();

            if let Some(events) = unpack(&response.payload, "events").as_array(){
                for event in events{
                    let event: Event = serde_json::from_value(event.clone()).unwrap();

                    if event.certification && event.completed{
                        window.0.lock().unwrap()
                            .eval(&format!("
                                document.getElementsByClassName('card-container')[0].innerHTML += `
                                    <div class='card'>
                                      <h2 class='card-title'>{}</h2>
                                      <p class='card-description'>{}</p>
                                      <p class='card-date'>{}</p>
                                    </div>`;
                            ", event.title, event.description, event.date))
                            .unwrap();
                    }
                }
            }
        }
        "EVENTS" =>{
            let mut signed_up = HashMap::new();
            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("GET_USER_EVENTS"), 
                    payload: String::new()
                }
            ).unwrap();

            let response = read_stream(&mut *STREAM.lock().unwrap());

            if let Some(events) = unpack(&response.payload, "events").as_array(){
                for event in events{
                    let event: Event = serde_json::from_value(event.clone()).unwrap();

                    signed_up.insert(event.title.to_owned(), event);
                }
            }

            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("GET_SHSM_EVENTS"), 
                    payload: String::new()
                }
            ).unwrap();

            let response = read_stream(&mut *STREAM.lock().unwrap());

            window.0.lock().unwrap()
                .eval("document.getElementsByClassName('event-container')[0].innerHTML = '';")
                .unwrap();

            if let Some(events) = unpack(&response.payload, "events").as_array(){
                for event in events{
                    let event: Event = serde_json::from_value(event.clone()).unwrap();

                    let button = if unsafe{ IS_TEACHER }
                        { "<button class='edit-event'>Edit Event</button>
                        <button class='view-signups' id='signup-button'>View Signups</button>" }
                        else
                        {
                            if signed_up.get(&event.title).is_some(){
                                "<button class='event-signup' id='signup-button'>Withdraw</button>"  
                            }
                            else{
                                "<button class='event-signup' id='signup-button'>Signup</button>"  
                            }
                        };

                    window.0.lock().unwrap()
                        .eval(&format!("
                            document.getElementsByClassName('event-container')[0].innerHTML += `
                              <div class='event'>
                                <h3 class='event-title'>{}</h3>
                                <p class='event-date'>Date: {}</p>
                                <p class='event-certified'>Certified: {}</p>
                                <p class='event-description'>{}</p>
                                {button} 
                              </div>
                            `;
                        ", event.title, event.date, event.certification, event.description))
                        .unwrap();
                }
            }
        }
        "CLASSLIST" =>{
            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("GET_CLASS_LIST"), 
                    payload: String::new()
                }
            ).unwrap();

            let response = read_stream(&mut *STREAM.lock().unwrap());

            window.0.lock().unwrap()
                .eval("
                    document.getElementById('students-container').innerHTML = `
                      <tr>
                        <th>Username</th>
                        <th>Actions</th>
                      </tr>`;
                ")
                .unwrap();

            if let Some(users) = unpack(&response.payload, "class_list").as_array(){
                for user in users{
                    let username: String = serde_json::from_value(user.clone()).unwrap();

                    println!("USER:");
                    println!("User Username: {}", username);
                    
                    window.0.lock().unwrap()
                        .eval(&format!("
                            var user = `
                              <tr>
                                <td class='username1'>{username}</td>
                                <td>
                                  <button class='edit-btn'>edit</button>
                                  <button class='addcrt-btn'>add certification</button>
                                  <button class='rmv-btn'>remove</button>
                                </td>
                              </tr>`;
                            document.getElementById('students-container').innerHTML += user;
                        "))
                        .unwrap();
                }
            }
        }
        "ANNOUNCEMENTS" =>{
            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("GET_ANNOUNCEMENTS"), 
                    payload: String::new()
                }
            ).unwrap();

            let response = read_stream(&mut *STREAM.lock().unwrap());

            window.0.lock().unwrap()
                .eval("document.getElementById('posted-announcement-container').innerHTML = '';")
                .unwrap();

            if let Some(announcements) = unpack(&response.payload, "announcements").as_array(){
                for announcement in announcements{
                    let announcement: Announcement = serde_json::from_value(announcement.clone()).unwrap();
                    
                    let button = if unsafe{ IS_TEACHER }
                        { "<button class='delete'>delete</button>" }
                        else
                        { "" };
                    window.0.lock().unwrap()
                        .eval(&format!("
                            var announcement = `
                            <div class='announcement'>
                                <div class='title'>{}</div>
                                <div class='description'>{}</div>
                                <div class='date'>{}</div>
                                {button}  
                            </div>`;
                            document.getElementById('posted-announcement-container').innerHTML += announcement;
                        ", announcement.title, announcement.description, announcement.date))
                        .unwrap();
                }
            }
        }
        _ =>(),
    }
}

#[tauri::command]
fn login_account(username: String, password: String, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("GET_ACCOUNT_KEYS"), 
            payload: json!({ "username": username }).to_string()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());
    if response.header == "GOOD"{
        const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
        let n_iter = NonZeroU32::new(100_000).unwrap();
        
        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
        let salt_key = unpack(&response.payload, "salt")
            .as_array()
            .unwrap()
            .into_iter()
            .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
            .collect::<Vec<u8>>();

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt_key,
            password.as_bytes(),
            &mut pbkdf2_hash,
        );

        write_stream(&mut *STREAM.lock().unwrap(), 
            Package { 
                header: String::from("VALIDATE_KEY"), 
                payload: json!({ "username": username, "hash": pbkdf2_hash.to_vec() }).to_string()
            }
        ).unwrap();

        let response = read_stream(&mut *STREAM.lock().unwrap());
        if response.header == "GOOD"{
            let page_name = unsafe{ 
                CURRENT_PAGE = String::from("ANNOUNCEMENTS");
                IS_TEACHER = unpack(&response.payload, "is_teacher").as_bool().unwrap();

                if IS_TEACHER
                { "teacher_home.html" }
                else
                { "student_home.html" }
            };

            window.0.lock().unwrap()
                .eval(&format!("window.location.replace('{page_name}');"))
                .unwrap();
        }
        else{
            MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
               .show(|_|{});
        }
    }
    else{
        MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
           .show(|_|{});
    }
}

#[tauri::command]
fn create_account(username: String, password: (String, String), course_code: String, is_teacher: bool, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("CHECK_ACCOUNT"), 
            payload: json!({ "username": username }).to_string()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());
    if response.header == "GOOD"{
        if password.0 == password.1{
            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("CHECK_CLASS"), 
                    payload: json!({ "course_code": course_code, "is_teacher": is_teacher }).to_string()
                }
            ).unwrap();

            let response = read_stream(&mut *STREAM.lock().unwrap());

            if response.header == "GOOD"{
                const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
                let n_iter = NonZeroU32::new(100_000).unwrap();
                let rng = rand::SystemRandom::new();

                let mut salt_key = [0u8; CREDENTIAL_LEN];
                rng.fill(&mut salt_key).unwrap();

                let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
                pbkdf2::derive(
                    pbkdf2::PBKDF2_HMAC_SHA512,
                    n_iter,
                    &salt_key,
                    password.0.as_bytes(),
                    &mut pbkdf2_hash,
                );
                
                let account = NewUser{ 
                    username: username.to_owned(), 
                    teacher: is_teacher,
                    hash: pbkdf2_hash.to_vec(), 
                    salt: salt_key.to_vec(),
                    code: course_code,
                };

                write_stream(&mut *STREAM.lock().unwrap(), 
                    Package { 
                        header: String::from("CREATE_ACCOUNT"), 
                        payload: serde_json::to_string(&account).unwrap()
                    }
                ).unwrap();

                let response = read_stream(&mut *STREAM.lock().unwrap());
                if response.header == "GOOD"{
                    window.0.lock().unwrap()
                        .eval("document.getElementById('sign-in').scrollIntoView({behavior: 'smooth', block: 'center', inline: 'center'});")
                        .unwrap();
                }
                else{
                    MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
                       .show(|_|{});
                }
            }
            else{
                MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
                   .show(|_|{});
            }
        }
        else{
            MessageDialogBuilder::new("ERROR ENCOUNTERED", "Passwords do not match! Please change to continue...")
               .show(|_|{});
        }
    }
    else{
        MessageDialogBuilder::new("ERROR ENCOUNTERED", unpack(&response.payload, "error").as_str().unwrap())
           .show(|_|{});
    }
}

#[tokio::main]
async fn main(){
    tauri::Builder::default()
        .setup(|app|{
            app.manage(WindowHandle(Mutex::new(app.get_window("main").unwrap())));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_account, login_account, add_announcement, sync_elements, add_event, remove_user, update_user, remove_announcement, get_user_events, certify_user, add_shsm_event, update_event, get_event_users, remove_event, modify_user_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
