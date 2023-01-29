#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
    net::TcpStream,
    sync::Mutex,
};
use netstruct::*;
use netstruct::models::NewUser;
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
use serde_json::{json, Value};

static mut CURRENT_PAGE: Page = Page::Login;
// const SOCKET: &str = "als-kou.ddns.net:7878";
const SOCKET: &str = "127.0.0.1:7878";
static STREAM: Lazy<Mutex<TcpStream>> = Lazy::new(||{
    Mutex::new(TcpStream::connect("127.0.0.1:7878").unwrap())
});

struct WindowHandle(Mutex<Window>);

fn sync_elements(){
    match unsafe{CURRENT_PAGE.clone()}{
        Page::Certifications =>{

        }
        Page::ShsmSelection =>{

        }
        Page::Events =>{

        }
        Page::Login =>{

        }
        Page::Home =>{

        }
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
            window.0.lock().unwrap()
                .eval("window.location.replace('home.html');")
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
fn create_account(username: String, password: String, course_code: String, is_teacher: bool, window: State<WindowHandle>){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("CHECK_ACCOUNT"), 
            payload: json!({ "username": username }).to_string()
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
            password.as_bytes(),
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

#[tokio::main]
async fn main(){
    tauri::Builder::default()
        .setup(|app|{
            app.manage(WindowHandle(Mutex::new(app.get_window("main").unwrap())));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_account, login_account])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
