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

    let mut content = String::new();
    let mut reader = BufReader::new(stream.try_clone()?);
    if let Some(c) = client {
        loop {
            let mut content_buf:Vec<u8> = Vec::new();
            match reader.read_until(b'', &mut content_buf) {
                Ok(r) => {
                    if r == 0{
                        break
                    }
                    content = String::from_utf8(content_buf)?;
                    println!("content in 6, {:?}", content);
                    if content.ends_with(""){
                        let _content = content.clone();
                        content = String::new();
                        if commands::run_command(c, &_content) {
                        } else {
                            c.add_content(cache::Role::Human, _content);
                            let prompt = c.migrate_content();
                            match openai::get(prompt).await {
                                Ok(mut res) => {
                                    res.push('');
                                    c.add_content(cache::Role::Robot, res.clone());
                                    println!("res::{res}");
                                    stream.write_all(res.as_bytes());
                                    stream.flush();
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

