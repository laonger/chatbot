use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::{
    commands,
    cache,
    openai,
};


pub async fn handle_connection(
    client_list: &mut cache::Clients, 
    mut stream: TcpStream
) -> openai::Result<()> {
    println!("connect in 3");

    let client = client_list.get_client(stream.peer_addr().unwrap().to_string());

    //println!("connect in 4");
    //let contents_list: Vec<_> = buf_reader
    //    .lines()
    //    .map(|result| result.unwrap())
    //    .collect();
    //println!("connect in 5");
    //let content = contents_list.join("\n");

    let mut reader = BufReader::new(stream.try_clone()?);
    if let Some(c) = client {
        loop {
            let mut content = String::new();
            let mut room_id = String::new();
            let mut content_buf:Vec<u8> = Vec::new();
            match reader.read_until(b'', &mut content_buf) {
                Ok(r) => {
                    if r == 0{
                        //client_list.remove_client(&c);
                        break
                    }
                    match String::from_utf8(content_buf)?
                        .replace("", "")
                        .split_once( "--$$__") {
                            Some((x, y)) => {
                                room_id = x.to_string();
                                content = y.to_string();
                            },
                            None => {
                                println!("need room_id");
                            }
                    }


                    //let _content = content.clone();
                    //content = String::new();
                    if commands::run_command(c, &room_id, &content) {
                    } else {
                        c.add_content(&room_id, cache::ContentUnit::user(content));
                        let messages = c.migrate_content(&room_id);
                        match openai::get(messages).await {
                            Ok(mut res) => {
                                res.push('');
                                c.add_content(
                                    &room_id,
                                    cache::ContentUnit::assistant(res.clone())
                                    );
                                res = vec![
                                    room_id, "--$$__".to_string(), res
                                ].join("");
                                stream.write_all(res.as_bytes());
                                stream.flush();
                            },
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        };
                    }
                },
                Err(e) => {
                    break
                }
            }
        }
    }
    println!("{:?}", client_list);

    Ok(())
}

