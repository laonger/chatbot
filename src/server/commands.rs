// Commands:
//   - new session
//
//

use std::{
    result::Result,
    net::TcpStream,
    fmt::Error,
};

use crate::cache;


fn execute(client: &mut cache::ClientUnit, cmd: &str) -> Result<(), Error> {
    match cmd {
        "/new" => {
            client.clear_content();
        },
        _ => {
            return Ok(());
        }
    }
    Ok(())
}

pub fn run_command(client: &mut cache::ClientUnit, s: &String) -> bool{
    let s = s.trim();
    if s.starts_with("/") {
        execute(client, s);
        true
    } else {
        false
    }
}
