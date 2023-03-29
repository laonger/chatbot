// Commands:
//   - new session
//
//
use std::{
    io::{prelude::*, BufReader},
    //net::TcpStream,
    sync::{
        Arc,
        //Mutex
    },
};

use tokio::{
    //time::sleep,
    //io::{
    //    self, 
    //    AsyncReadExt,
    //    AsyncWriteExt,
    //},
    sync::Mutex,
    //net::{
    //    TcpListener,
    //    TcpStream,
    //},
};

use std::{
    result::Result,
    net::TcpStream,
    fmt::Error,
};

use crate::cache;


pub fn run_command(
    client: &mut cache::ClientUnit,
    //client_list: &mut Arc<Mutex<cache::Clients>>,
    room_id: &String,
    s: &String
) -> Result<String, Error>{
    let s = s.strip_suffix("\n").unwrap_or(s).trim();
    match s.split_once(" ") {
        None => {
            match s.clone() {
                "/new" => {
                    client.clear_content(room_id);
                    return Ok("history cleared".to_string())
                },
                "/rooms" => {
                    let mut r = String::new();
                    for i in client.rooms() {
                        r.push_str(format!("{i}, ").as_str());
                    }
                    if r.is_empty(){
                        r = "no rooms".to_string();
                    }
                    return Ok(r);
                },
                _ => {
                    return Err(Error);
                }
            }
        },
        Some(("/room_content", room_id)) => {
            let mut r = String::new();
            for c in client.migrate_content(&room_id.to_string()) {
                match c {
                    cache::ContentUnit::user(x) => {
                        r.push_str(x.as_str())
                    },
                    _ => {
                    }
                };
            }
            r.push('\n');
            return Ok(r)
        },
        _ => {
            return Err(Error);
        }
    }
}
