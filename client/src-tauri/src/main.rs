#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    collections::HashMap, sync::Mutex,
};
use netstruct::*;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

static mut CURRENT_PAGE: Page = Page::Login;
const SOCKET: &str = "als-kou.ddns.net:7878";
static STREAM: Mutex<Option<TcpStream>> = Mutex::new(None);

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

fn insert_in_database(key: &str, val: &str){}

#[tauri::command]
fn create_account(username: String, password: String){
    let request = "CHECK_ACCOUNT\n\n";
    let mut stream = TcpStream::connect(SOCKET).unwrap();
    stream.write_all(request.as_bytes()).unwrap();

    let response = package_stream(&mut stream);

    if &response[0] == "!EXISTS"{
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

        let account = Account{ username, hash, salt, };

        let request = format!("CREATE_ACCOUNT\n{}\n\n", serde_json::to_string(&account).unwrap());
        println!("REQUEST: \n{}", request);
        stream.write_all(request.as_bytes()).unwrap();
        stream.flush().unwrap();

    }
}

#[tokio::main]
async fn main(){
    // create_account(String::from("Koudocko"), String::from("fajdsxD16612369E"));

    let handle = *STREAM.lock().unwrap(); 
    handle = Some(TcpStream::connect(SOCKET).unwrap());
    stream.write_all(request.as_bytes()).unwrap();
    write_stream(&)

    let buf_reader = BufReader::new(&mut stream);
    let response: Vec<_> = buf_reader
        .lines()
        .map(|result|{
            if let Ok(result) = result{
                result
            }
            else{
                String::new()
            }
        })
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Response: {:#?}", response);

    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
