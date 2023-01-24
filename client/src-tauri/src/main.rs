#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
    net::TcpStream,
    sync::Mutex,
};
use netstruct::*;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
use once_cell::sync::Lazy;

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

fn login_account(username: String, password: String){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("GET_ACCOUNT_KEYS"), 
            payload: username.to_owned()
        }
    ).unwrap();
}

#[tauri::command]
fn create_account(username: String, password: String){
    write_stream(&mut *STREAM.lock().unwrap(), 
        Package { 
            header: String::from("CHECK_ACCOUNT"), 
            payload: username.to_owned()
        }
    ).unwrap();

    let response = read_stream(&mut *STREAM.lock().unwrap());

    if response.payload == "!EXISTS"{
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
        
        let mut hash = [[0_u8; 32]; 2];
        for i in 0..64{
            hash[i / 32][i % 32] = pbkdf2_hash[i];
        }

        let mut salt = [[0_u8; 32]; 2];
        for i in 0..64{
            salt[i / 32][i % 32] = salt_key[i];
        }

        let account = Account{ username: username.to_owned(), hash, salt, };

        write_stream(&mut *STREAM.lock().unwrap(), 
            Package { 
                header: String::from("CREATE_ACCOUNT"), 
                payload: serde_json::to_string(&account).unwrap()
            }
        ).unwrap();
    }
}

#[tokio::main]
async fn main(){
    create_account(String::from("Koudocko"), String::from("fajdsxD16612369E"));

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
