use std::io::prelude::*;
use std::net::TcpStream;
use std::io::{stdin,stdout,Write, BufReader};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    //stream.write(&[1])?;
    //stream.read(&mut [0; 128])?;
    loop {
        let mut user_input = String::new();
        println!("Human: ");
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
                let content = format!("AI: {}", String::from_utf8(res_buf).unwrap());
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
