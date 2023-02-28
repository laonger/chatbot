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

    let mut content:String = String::new();
    if let Some(c) = client {
        loop {
            match stream.read_to_string(&mut content) {
                Ok(r) => {
                    println!("content in 6, {:?}", content);
                    if r == 0 {
                        break
                    }
                    if content.ends_with("\n\n"){
                        let _content = content.clone();
                        content = String::new();
                        if commands::run_command(c, &_content) {
                        } else {
                            c.add_content(cache::Role::Human, _content);
                            let prompt = c.migrate_content();
                            match openai::get(prompt).await {
                                Ok(res) => {
                                    c.add_content(cache::Role::Robot, res.clone());
                                    println!("res::{res}");
                                    stream.write(res.as_bytes());
                                },
                                Err(e) => {
                                }
                            };

                        }
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

