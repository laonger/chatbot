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


fn execute(
    client: &mut cache::ClientUnit, room_id: &String, cmd: &str
) -> Result<(), Error> {
    match cmd {
        "/new" => {
            client.clear_content(room_id);
        },
        _ => {
            return Ok(());
        }
    }
    Ok(())
}

pub fn run_command(
    client: &mut cache::ClientUnit, room_id: &String, s: &String
) -> bool{
    let s = s.trim();
    if s.starts_with("/") {
        execute(client, room_id, s);
        true
    } else {
        false
    }
}
