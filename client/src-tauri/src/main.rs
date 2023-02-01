#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
    net::TcpStream,
    sync::Mutex,
};
use netstruct::*;
use netstruct::models::{NewUser, Announcement, Event};
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

            // // window.0.lock().unwrap()
            // //     .eval("document.getElementById('posted-announcement-container').innerHTML = '';")
            //     // .unwrap();

            if let Some(certifications) = unpack(&response.payload, "certifications").as_array(){
                for certification in certifications{
                    let certification: Event = serde_json::from_value(certification.clone()).unwrap();
                    
                    println!("CERTIFICATION:");
                    println!("Certification Title: {}", certification.title);
                    println!("Certification Description: {}", certification.description);
                    println!("Certification Date: {}", certification.date);
                    println!("Certification Completed: {}", certification.completed);
                //     // window.0.lock().unwrap()
                //     //     .eval(&format!("
                //     //         var announcement = `
                //     //         <div class='announcement'>
                //     //             <div class='title'>{}</div>
                //     //             <div class='description'>{}</div>
                //     //         </div>`;
                //     //         document.getElementById('posted-announcement-container').innerHTML += announcement;
                //     //     ", announcement.title, announcement.description))
                //     //     .unwrap();
                }
            }
        }
        "EVENTS" =>{
            write_stream(&mut *STREAM.lock().unwrap(), 
                Package { 
                    header: String::from("GET_SHSM_EVENTS"), 
                    payload: String::new()
                }
            ).unwrap();

            let response = read_stream(&mut *STREAM.lock().unwrap());

            // window.0.lock().unwrap()
            //     .eval("document.getElementById('posted-announcement-container').innerHTML = '';")
                // .unwrap();

            if let Some(events) = unpack(&response.payload, "events").as_array(){
                for event in events{
                    let event: Event = serde_json::from_value(event.clone()).unwrap();

                    println!("EVENT:");
                    println!("Event Title: {}", event.title);
                    println!("Event Description: {}", event.description);
                    println!("Event Date: {}", event.date);
                    println!("Event Certification: {}", event.certification);
                    
                //     // window.0.lock().unwrap()
                //     //     .eval(&format!("
                //     //         var announcement = `
                //     //         <div class='announcement'>
                //     //             <div class='title'>{}</div>
                //     //             <div class='description'>{}</div>
                //     //         </div>`;
                //     //         document.getElementById('posted-announcement-container').innerHTML += announcement;
                //     //     ", announcement.title, announcement.description))
                //     //     .unwrap();
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

            // window.0.lock().unwrap()
            //     .eval("document.getElementById('posted-announcement-container').innerHTML = '';")
                // .unwrap();

            if let Some(users) = unpack(&response.payload, "class_list").as_array(){
                for user in users{
                    let username: String = serde_json::from_value(user.clone()).unwrap();

                    println!("USER:");
                    println!("User Username: {}", username);
                    
                //     // window.0.lock().unwrap()
                //     //     .eval(&format!("
                //     //         var announcement = `
                //     //         <div class='announcement'>
                //     //             <div class='title'>{}</div>
                //     //             <div class='description'>{}</div>
                //     //         </div>`;
                //     //         document.getElementById('posted-announcement-container').innerHTML += announcement;
                //     //     ", announcement.title, announcement.description))
                //     //     .unwrap();
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
                    
                    window.0.lock().unwrap()
                        .eval(&format!("
                            var announcement = `
                            <div class='announcement'>
                                <div class='title'>{}</div>
                                <div class='description'>{}</div>
                                <div class='date'>{}</div>
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
        .invoke_handler(tauri::generate_handler![create_account, login_account, add_announcement, sync_elements, add_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
