use std::{
    //net::TcpListener,
    net::SocketAddr,
    //thread::sleep,
//    time,
    sync::{
        Arc,
        //Mutex
    },
    ops::DerefMut,
};

//use async_std::{
//    task::sleep,
//    net::{
//        TcpListener,
//        TcpStream,
//    },
//};

//use futures::lock;
use tokio::{
//    time::sleep,
//    io::{
//        self, 
//        AsyncReadExt,
//        AsyncWriteExt,
//    },
    sync::Mutex,
    net::{
        TcpListener,
        TcpStream,
    },
};


mod handler;
mod commands;
mod cache;
mod openai;

type ShareCLientList = Arc<Mutex<cache::Clients>>;

#[tokio::main]
//#[async_std::main]
async fn main() -> openai::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:7878").await?;

    let client_list:ShareCLientList = Arc::new(Mutex::new(cache::Clients::new()));
    //let mut client_list = cache::Clients::new();

    loop {
        println!("1111");
        //let mut tcpstream:TcpStream;
        //let mut address:String;
        
        //let (mut tcpstream, address) = listener.accept().await.unwrap();
        let (mut tcpstream, address) = match listener.accept().await {
            Ok((tcpstream, address)) => {
                (tcpstream, address)
            },
            Err(_) => {
                continue
            }
        };
        //let address = address.to_string();
        let mut client_list = client_list.clone();

        //drop(client_list);
        println!("2222");

        tokio::spawn( async move {
            loop{
                println!("7777");
                match handler::handle_connection(&mut client_list, &mut tcpstream).await {
                    Ok(_) => {
                    },
                    Err(e) => {
                        break
                    }
                };
            };
        });
    };
    //Ok(())
}

