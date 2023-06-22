use std::{
    sync::Arc,
    //sync::Mutex,
    env,
};
use log::{
    info,
    debug,
};

use tokio::{
    sync::Mutex,
    io::{
        self, 
        AsyncReadExt,
        AsyncWriteExt,
    },
    net::{
        TcpListener,
    },
};



mod handler;
mod commands;
mod cache;
mod openai;

type ShareCLientList = Arc<Mutex<cache::Clients>>;

#[tokio::main]
async fn main() -> openai::Result<()>{
    let port = match env::var("PORT") {
        Ok(x) => {
            x
        },
        Err(_) => {
            "7878".to_string()
        }
    };
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;


    let client_list = Arc::new(Mutex::new(cache::Clients::new()));

    loop {
        println!("1");
        // 拿到连接
        let (mut tcpstream, address) = match listener.accept().await {
            Ok((mut tcpstream, address)) => {
                info!("Connection in...");
                //tcpstream.write_all("Human > ".as_bytes()).await?;
                (tcpstream, address)
            },
            Err(_) => {
                continue
            }
        };
        let mut client_list = client_list.clone();

        tokio::spawn(async move {
            loop{
                println!("2");
                match handler::handle_connection(&mut client_list, &mut tcpstream).await {
                    Ok(_) => {
                    },
                    Err(e) => match e.description() {
                        "connection aborted" => {
                            eprintln!("{:?}", e);
                            break
                        },
                        _ => {
                            eprintln!("{:?}", e);
                            break
                        }
                    }
                };
                println!("3");
            };
        });
    };
}

