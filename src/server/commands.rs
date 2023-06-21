// Commands:
//   - new session
//
//
use std::{
    io::{prelude::*, BufReader},
    str::FromStr,
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
    //sync::Mutex,
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

use crate::{
    cache,
    openai,
};


pub async fn run_command(
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
                    println!("lock: 3.11");
                    for i in client.rooms() {
                        r.push_str(format!("{i}, ").as_str());
                    }
                    println!("lock: 3.12");
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
        Some(("/image", args_str)) => {
            let args:Vec<&str> = args_str.splitn(3, " ").collect();
            let (n, size, prompt) = match args.len() {
                1 => {
                    (1, "512x512".to_string(), args[0].to_string())
                },
                2 => {
                    let arg_0 = args[0];
                    match arg_0 {
                        "1024x1024" | "512x512" | "256x256" => {
                            (1, arg_0.to_string(), args[1].to_string())
                        },
                        _ => {
                            match i32::from_str(arg_0) {
                                Ok(n) => {
                                    (n, "512x512".to_string(), args[1].to_string())
                                },
                                Err(_) => {
                                    (1, "512x512".to_string(), args_str.to_string())
                                }
                            }
                        }
                    }
                },
                3 => {
                    let mut n = i32::from_str(args[0]).unwrap();
                    if n >= 4 {
                        n = 4
                    }
                    let size = args[1].to_string();
                    (n, size, args[2].to_string())
                }
                _ => {
                    (1, "512x512".to_string(), args_str.to_string())
                }
            };
            let r = match openai::draw(prompt, n, size).await {
                Ok(r) => {
                    r
                },
                _ => {
                    "".to_string()
                }
            };
            return Ok(r)
        },
        _ => {
            return Err(Error);
        }
    }
}
