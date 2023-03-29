use std::{
    io::{prelude::*, BufReader},
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
    net::{
        TcpListener,
        TcpStream,
    },
};

use crate::{
    commands,
    cache,
    openai,
};


pub async fn handle_connection (
    client_list: &mut Arc<Mutex<cache::Clients>>, 
    stream: &mut TcpStream
) -> openai::Result<()> {
    
    let address = stream.peer_addr().unwrap().to_string();

    let buf_size = 8;
    let mut temp_buf:Vec<u8> = vec![0; buf_size];
    let mut content_buf = vec![];
    
    loop {
        match stream.read(&mut temp_buf).await {
            Ok(0) => {
                return Err(Box::new(
                        io::Error::from(io::ErrorKind::ConnectionAborted)
                        ));
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

    //println!("content: {:?}", String::from_utf8(content_buf.clone()));
    let (room_id, content) = match String::from_utf8(content_buf.clone())?
        .replace("", "")
        .split_once("--$$__") {
            Some((x, y)) => {
                (
                    x.to_string(),
                    y.to_string()
                )
            },
            None => {
                //println!("need room_id");
                //// TODO return err
                //stream.write_all("data error".as_bytes()).await?;
                //("".to_string(), "".to_string())
                println!("p1: {}", String::from_utf8(content_buf.clone())?);
                // nc connections
                ("1".to_string(), String::from_utf8(content_buf)?)
            }
    };

    if content.replace("\n", "").is_empty() {
        return Ok(())
    }

    let mut messages: Vec<cache::ContentUnit> = Vec::new();
    {
        let mut client_list = client_list.lock().await;
        let client = match client_list.get_client(address.clone()) {
            Some(c) => {
                c
            },
            None => {
                client_list.add_client(address.clone());
                client_list.get_client(address.clone()).unwrap()
            }
        };
        match commands::run_command(client, &room_id, &content) {
            Ok(m) => {
                let mut res = m;
                res.push('\n');
                if room_id != "1".to_string() && room_id != "2".to_string(){
                    res = vec![
                        room_id, "--$$__".to_string(), res
                    ].join("");
                } else {
                    res = format!("AI > {res}\nHuman > ");
                }
                stream.write_all(res.as_bytes()).await?;
                stream.flush().await?;
                return Ok(())
            },
            Err(_) => {
            }

        }
        client.add_content(&room_id, cache::ContentUnit::user(content));
        messages = client.migrate_content(&room_id);
    }
    {
        match openai::get(messages).await {
            Ok(mut res) => {
                res.push('');

                let mut client_list = client_list.lock().await;
                let client = client_list.get_client(address.clone()).unwrap();
                client.add_content(&room_id, 
                    cache::ContentUnit::assistant(res.clone())
                );
                drop(client);

                if room_id != "1".to_string() && room_id != "2".to_string(){
                    res = vec![
                        room_id, "--$$__".to_string(), res
                    ].join("");
                    res.push('\n');
                } else {
                    res = format!("AI > {res}\nHuman > ");
                    println!("a1: {}", res.clone());
                }
                stream.write_all(res.as_bytes()).await?;
                stream.flush().await?;
            },
            Err(e) => {
                println!("{:?}", e);
                // TODO return err
            }
        }
    }

    Ok(())
}

