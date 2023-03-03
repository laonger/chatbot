use std::net::TcpListener;

mod handler;
mod commands;
mod cache;
mod openai;

#[tokio::main]
async fn main() -> openai::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    let mut client_list = cache::Clients::new();

    for stream in listener.incoming(){
        println!("connect in");
        match stream {
            Ok(stream) => {
                println!("connect in 2");
                let address = stream.peer_addr()?.to_string();
                client_list.add_client(address);
                handler::handle_connection(&mut client_list, stream).await?;
            },
            Err(e) => {
                println!("sream error: {:?}", e);
            }
        }
    }
    Ok(())
}

