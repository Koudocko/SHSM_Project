#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    collections::HashMap,
};
use netstruct::*;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

static mut CURRENT_PAGE: Page = Page::Login;
const SOCKET: &str = "als-kou.ddns.net:7878";

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
fn create_account(account: Account){
    let request = "EXISTS\n";
    let mut stream = TcpStream::connect(SOCKET).unwrap();
    stream.write_all(request.as_bytes()).unwrap();

    let response = package_stream(&mut stream);

    if &response[0] == "BAD"{
        const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
        let n_iter = NonZeroU32::new(100_000).unwrap();
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; CREDENTIAL_LEN];
        rng.fill(&mut salt).unwrap();

        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            account.password.as_bytes(),
            &mut pbkdf2_hash,
        );

        
        println!("Salt: {}", HEXUPPER.encode(&salt));
        println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));
    }
}

#[tokio::main]
async fn main(){
    let request = "ping\n\n";

    let mut stream = TcpStream::connect(SOCKET).unwrap();
    stream.write_all(request.as_bytes()).unwrap();

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

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
