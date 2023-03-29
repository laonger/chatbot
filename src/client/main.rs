use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{stdin,stdout,Write, BufReader};
use std::env;

fn main() -> std::io::Result<()> {
    //let mut stream = TcpStream::connect("208.115.245.46:7878")?;
    let args: Vec<String> = env::args().collect();
    let mut ip = "127.0.0.1";
    let mut port = "7878";
    if args.len() >1 {
        ip = &args[1];
        port = &args[2];
    }

    let mut stream = TcpStream::connect(format!("{ip}:{port}"))?;

    //stream.write(&[1])?;
    //stream.read(&mut [0; 128])?;
    loop {
        let mut user_input = "2--$$__".to_string();
        print!("Human: ");
        stdin().read_line(&mut user_input).expect("");
        //println!("{user_input}");
        user_input.push('');
        let send_bytes = user_input.as_bytes();
        stream.write_all(send_bytes)?;
        stream.flush()?;
        let mut res = String::new();
        let mut res_reader = BufReader::new(stream.try_clone()?);
        let mut res_buf:Vec<u8> = Vec::new();
        match res_reader.read_until(b'', &mut res_buf) {
            Ok(_) => {
                let content = format!("AI: {}", String::from_utf8(res_buf).unwrap().replace("", ""));
                for l in content.split("\n"){
                    println!("{l}");
                }
            },
            Err(_) => {
                break
            }
        };
    }
    Ok(())
}
