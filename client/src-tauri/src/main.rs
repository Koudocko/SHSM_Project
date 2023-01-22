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

#[tokio::main]
async fn main(){
    let request = "ping\n\n";

    let mut stream = TcpStream::connect(SOCKET).unwrap();
    stream.write_all(request.as_bytes()).unwrap();

    let buf_reader = BufReader::new(&mut stream);
    let response: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Response: {:#?}", response);

    // tauri::Builder::default()
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
