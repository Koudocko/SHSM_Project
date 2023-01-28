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
use tauri::api::dialog::MessageDialogBuilder;

static mut CURRENT_PAGE: Page = Page::Login;
// const SOCKET: &str = "als-kou.ddns.net:7878";
const SOCKET: &str = "127.0.0.1:7878";
static STREAM: Lazy<Mutex<TcpStream>> = Lazy::new(||{
    Mutex::new(TcpStream::connect("127.0.0.1:7878").unwrap())
});

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
fn login_account(username: String, password: String){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("GET_ACCOUNT_KEYS"), 
            payload: username.to_owned()
        }
    ).unwrap();
}

#[tauri::command]
fn create_account(username: String, password: String, is_teacher: bool){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("CHECK_ACCOUNT"), 
            payload: username.to_owned()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());
    if response.header == "GOOD"{
        println!("good1");
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
            code: None,
        };

        write_stream(&mut *STREAM.lock().unwrap(), 
            Package { 
                header: String::from("CREATE_ACCOUNT"), 
                payload: serde_json::to_string(&account).unwrap()
            }
        ).unwrap();

        let response = read_stream(&mut *STREAM.lock().unwrap());
        if response.header == "BAD"{
           println!("bad2");
        }
    }
    else{
        println!("bad1");
        MessageDialogBuilder::new("Username", response.payload)
           .show(|_|{});
    }
}

#[tokio::main]
async fn main(){
    // create_account(String::from("Koudocko"), String::from("fajdsxD16612369E"), true);

    // write_stream(&mut *STREAM.lock().unwrap(), 
    //     Package { 
    //         header: String::from("CHECK_ACCOUNT\nbozo"), 
    //         payload: String::from("bro") 
    //     }
    // ).unwrap();

    // let response = read_stream(&mut *STREAM.lock().unwrap());
    // println!("Response: {:?}", response);

    // write_stream(&mut *STREAM.lock().unwrap(), 
    //     Package { 
    //         header: String::from("CHECK_ACCOUNT\nbozo"), 
    //         payload: String::from("bro") 
    //     }
    // ).unwrap();

    // let response = read_stream(&mut *STREAM.lock().unwrap());
    // println!("Response: {:?}", response);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_account, login_account])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
