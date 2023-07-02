use std::{
    io::{prelude::*, BufReader},
    error::Error as STDError,
    //net::TcpStream,
    sync::{
        Arc,
        //Mutex
    },
};

//use async_std::{
//    task::sleep,
//    net::{
//        TcpListener,
//        TcpStream,
//    },
//};

use tokio::{
    time::sleep,
    io::{
        self, 
        AsyncReadExt,
        AsyncWriteExt,
    },
    sync::Mutex,
    net::TcpStream,
};

use simple_openai::RoleType;
use simple_openai as openai;

use crate::{
    commands,
    cache,
};


pub async fn pull_out_content(stream: &mut TcpStream) 
    -> openai::Result<(String, String)> {

    let buf_size = 13;
    let mut temp_buf:Vec<u8> = vec![0; buf_size];
    let mut content_buf = vec![];

    loop { // 反复读取，直到没有新的数据为止
        stream.readable().await.unwrap();
        match stream.try_read(&mut temp_buf) {
            Ok(0) => {
                return Err(
                    io::Error::from(io::ErrorKind::ConnectionAborted).into()
                );
            },
            Ok(r) => {
                content_buf.extend_from_slice(&temp_buf[..r]);
                temp_buf = vec![0; buf_size];
                if r != buf_size {
                    break
                }
                continue
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            },
            Err(e) => {
                println!("eeeee, {:?}", e);
                return Err(e.into());
            }
        }
    }
    let (room_id, content) = match String::from_utf8(content_buf.clone()) {
        Ok(r) => match r.replace("", "") .split_once("--$$__") {
            Some((x, y)) => {
                (
                    x.to_string(),
                    y.to_string()
                )
            },
            None => {
                // nc connections
                println!("nc: {}", 
                    String::from_utf8(content_buf.clone()).unwrap()
                    );
                ("1".to_string(), String::from_utf8(content_buf).unwrap())
            }
        },
        Err(e) => {
            return Err(e.into())
        }

    };

    if content.replace("\n", "").is_empty() {
        return Err("no content".into())
    }
    return Ok((room_id, content))
    
}

pub async fn handle_connection (
    client_list: &mut Arc<Mutex<cache::Clients>>, 
    stream: &mut TcpStream
) -> openai::Result<()> {
    
    println!("h1");
    
    let address = match stream.peer_addr(){
        Ok(addr) => {
            addr.to_string()
        },
        Err(e) => {
            return Err(e.into())
        }
    };
    
    let (room_id, content)  = match pull_out_content(stream).await {
        Ok((r, c)) => {
            (r, c)
        },
        Err(e) => match e.description() {
            "connection aborted" => {
                let mut client_list = client_list.lock().await;
                client_list.remove_client(address);
                return Err(e)
            },
            "no content" => {
                eprintln!("{:?}", e);
                return Ok(())
            },
            _ => {
                eprintln!("{:?}", e);
                let mut client_list = client_list.lock().await;
                client_list.remove_client(address);
                return Err(e)
                //("".to_string(), "".to_string())
            }
        }
    };

    println!("room_id old: {}", room_id);

    let mut messages: Vec<openai::RoleType> = Vec::new();
    {
        let mut client_list = client_list.lock().await;
        let client = match (&mut client_list).get_client(address.clone()) {
            Some(c) => {
                c
            },
            None => {
                client_list.add_client(address.clone());
                client_list.get_client(address.clone()).unwrap()
            }
        };
        if let Ok(m) = commands::run_command(client, &room_id, &content).await {
            let mut res = m;
            res.push('\n');
            if room_id != "1".to_string() && room_id != "2".to_string(){
                res = vec![
                    room_id, "--$$__".to_string(), res
                ].join("");
            } else {
                res = format!("AI > {res}\nHuman > ");
            }
            println!("command answer: {}", res);
            res.push('');
            stream.write_all(res.as_bytes()).await?;
            stream.flush().await?;
            return Ok(())
        }
        client.add_content(&room_id, openai::RoleType::user(content));
        messages = client.migrate_content(&room_id);
        drop(client);
    }
    {
        match openai::ask(messages).await {
            Ok(mut res) => {

                let mut client_list = client_list.lock().await;
                let client = client_list.get_client(address.clone()).unwrap();
                client.add_content(&room_id, 
                    openai::RoleType::assistant(res.clone())
                );
                drop(client);

                println!("room_id new: {}", room_id);
                if room_id != "1".to_string() && room_id != "2".to_string(){
                    res = vec![
                        room_id, "--$$__".to_string(), res
                    ].join("");
                    res.push('\n');
                } else {
                    res = format!("AI > {res}\nHuman > ");
                    println!("a1: {}", res.clone());
                }
                res.push('');
                stream.writable().await.unwrap();
                stream.write_all(res.as_bytes()).await;
                stream.flush().await;
            },
            Err(e) => {
                println!("{:?}", e);
                // TODO return err
            }
        }
    }
    println!("lock: 11");

    Ok(())
}

