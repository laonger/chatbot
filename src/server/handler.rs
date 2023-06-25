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

use crate::{
    commands,
    cache,
    openai,
};


pub async fn pull_out_content(stream: &mut TcpStream) 
    -> openai::Result<(String, String)> {

    let buf_size = 8;
    let mut temp_buf:Vec<u8> = vec![0; buf_size];
    let mut content_buf = vec![];

    println!("p1");
    loop { // 反复读取，直到没有新的数据为止
        match stream.read(&mut temp_buf).await {
            Ok(0) => {
                println!("p1.1");
                return Err(
                    io::Error::from(io::ErrorKind::ConnectionAborted).into()
                );
            },
            Ok(r) => {
                println!("p1.2.1");
                content_buf.extend_from_slice(&temp_buf[..r]);
                println!("p1.2.2");
                temp_buf = vec![0; buf_size];
                println!("p1.2.3");
                if r != buf_size {
                    println!("p1.2.4");
                    break
                }
                println!("p1.2.5");
                continue
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("p1.3");
                continue;
            },
            Err(e) => {
                println!("eeeee, {:?}", e);
                return Err(e.into());
            }
        }
    }
    println!("p2");
    let (room_id, content) = match String::from_utf8(content_buf.clone()) {
        Ok(r) => match r.replace("", "") .split_once("--$$__") {
            Some((x, y)) => {
                println!("p2.1");
                (
                    x.to_string(),
                    y.to_string()
                )
            },
            None => {
                // nc connections
                println!(
                    "p2.2: {}", 
                    String::from_utf8(content_buf.clone()).unwrap()
                    );
                ("1".to_string(), String::from_utf8(content_buf).unwrap())
            }
        },
        Err(e) => {
            println!("p2.3, {:?}", e);
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
    println!("h2");
    
    let (room_id, content)  = match pull_out_content(stream).await {
        Ok((r, c)) => {
            (r, c)
        },
        Err(e) => match e.description() {
            "connection aborted" => {
                println!("h2.1");
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
                println!("h2.3");
                let mut client_list = client_list.lock().await;
                client_list.remove_client(address);
                return Err(e)
                //("".to_string(), "".to_string())
            }
        }
    };
    println!("h3");

    println!("room_id old: {}", room_id);

    let mut messages: Vec<cache::ContentUnit> = Vec::new();
    {
        println!("lock: 1");
        let mut client_list = client_list.lock().await;
        println!("lock: 2");
        let client = match (&mut client_list).get_client(address.clone()) {
            Some(c) => {
                c
            },
            None => {
                client_list.add_client(address.clone());
                client_list.get_client(address.clone()).unwrap()
            }
        };
        println!("lock: 3");
        if let Ok(m) = commands::run_command(client, &room_id, &content).await {
            println!("lock: 3.1");
            let mut res = m;
            res.push('\n');
            if room_id != "1".to_string() && room_id != "2".to_string(){
                res = vec![
                    room_id, "--$$__".to_string(), res
                ].join("");
            } else {
                res = format!("AI > {res}\nHuman > ");
            }
            println!("lock: 3.2");
            println!("command answer: {}", res);
            res.push('');
            stream.write_all(res.as_bytes()).await?;
            stream.flush().await?;
            println!("lock: 3.3");
            return Ok(())
        }
        println!("lock: 4");
        client.add_content(&room_id, cache::ContentUnit::user(content));
        messages = client.migrate_content(&room_id);
        println!("lock: 4");
        drop(client);
    }
    {
        match openai::ask(messages).await {
            Ok(mut res) => {

                println!("lock: 5");
                let mut client_list = client_list.lock().await;
                println!("lock: 6");
                let client = client_list.get_client(address.clone()).unwrap();
                println!("lock: 7");
                client.add_content(&room_id, 
                    cache::ContentUnit::assistant(res.clone())
                );
                println!("lock: 8");
                drop(client);
                println!("lock: 9");

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
                stream.write_all(res.as_bytes()).await;
                stream.flush().await;
                println!("lock: 10");
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

