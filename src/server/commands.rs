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


fn execute(
    client: &mut cache::ClientUnit,
    room_id: &String,
    cmd: &str
) -> Result<String, Error> {
    match cmd {
        "/new" => {
            client.clear_content(room_id);
            return Ok("history cleared".to_string())
        },
        "/clients" => {
            return Ok("".to_string())
        },
        _ => {
            return Err(Error);
        }
    }
}

pub fn run_command(
    client: &mut cache::ClientUnit,
    room_id: &String,
    s: &String
) -> Result<String, Error>{
    let s = s.trim();
    if s.starts_with("/") {
        return execute(client, room_id, s);
    } else {
        Err(Error)
    }
}
