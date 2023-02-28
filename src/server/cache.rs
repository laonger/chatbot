use std::collections::HashMap;
use std::convert::AsMut;


enum Type {
    text,
    command
}

#[derive(Debug)]
pub enum Role {
    Robot,
    Human
}

#[derive(Debug)]
pub struct ContentUnit {
    role: Role,
    content: String,
}

#[derive(Debug)]
pub struct ClientUnit {
    addr: String,
    contents: Vec<ContentUnit>,
}


impl ClientUnit {
    pub fn new(addr:String) -> Self {
        Self { 
            addr,
            contents: Vec::new()
        }
    }

    pub fn add_content(&mut self, role: Role, content: String) {
        self.contents.push(ContentUnit{
            role,
            content
        })
    }

    pub fn migrate_content(&self) -> String {
        let mut s:Vec<String> = Vec::new();
        for i in &(self.contents) {
            let mut _s:String = match i.role {
                Role::Robot => {
                    "AI: ".to_string()
                },
                Role::Human => {
                    "Human: ".to_string()
                }
            };
            _s.push_str(i.content.clone().as_str());
            s.push(_s);
        }
        s.join("\n")
    }

    pub fn clear_content(&mut self) {
        self.contents = Vec::new();
    }
}

#[derive(Debug)]
pub struct Clients {
    data: HashMap<String, ClientUnit>,
}

impl  Clients {
    pub fn new() -> Self {
        Self { 
            data: HashMap::new()
        }
    }

    pub fn add_client(&mut self, addr:String){
        if !self.data.contains_key(&addr) {
            let client = ClientUnit::new(addr.clone());
            self.data.insert(addr.clone(), client);
        }
    }

    pub fn get_client(&mut self, addr:String) -> Option<&mut ClientUnit>{
        self.data.get_mut(&addr)

    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_unit_migrate_content_test() {
        let mut cu = ClientUnit::new("".to_string());
        cu.add_content(Role::Human, "hihihi".to_string());
        cu.add_content(Role::Robot, "hi".to_string());
        assert_eq!(cu.migrate_content(), "Human: hihihi\nAI: hi".to_string());
    }
}
