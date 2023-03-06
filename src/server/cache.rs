use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag="role", content="content")]
pub enum ContentUnit {
    Robot(String),
    Human(String),
    System(String),
}

//#[derive(Debug)]
//pub struct ContentUnit {
//    role: Role,
//    content: String,
//}

#[derive(Debug, Clone)]
pub struct ClientUnit {
    addr: String,
    contents: HashMap<String, Vec<ContentUnit>>,
}


impl ClientUnit {
    pub fn new(addr:String) -> Self {
        Self { 
            addr,
            contents: HashMap::new()
        }
    }

    //pub fn init_content(&self) {
    //    let prompt = ("The following is a conversation with an AI Robot. The Robot is helpful, creative, clever, and very friendly. ");
    //    
    //}

    pub fn add_content(&mut self, room_id: &String, content:ContentUnit) {
        match self.contents.get_mut(room_id) {
            Some(c) => {
                c.push(content)
            },
            None => {
                let c_l = vec![content];
                self.contents.insert(room_id.clone(), c_l);
            }
        }
    }

    // TODO 
    //pub fn migrate_content(&self) -> String {
    //    let mut s:Vec<String> = Vec::new();
    //    for i in &(self.contents) {
    //        let mut _s:String = match i.role {
    //            Role::Robot => {
    //                "AI: ".to_string()
    //            },
    //            Role::Human => {
    //                "Human: ".to_string()
    //            }
    //        };
    //        _s.push_str(i.content.clone().as_str());
    //        s.push(_s);
    //    }
    //    s.join("\n")
    //}
    
    pub fn migrate_content(&mut self, room_id: &String) -> Vec<ContentUnit> {
        match self.contents.get(room_id).as_mut() {
            Some(&mut x) => {
                return x.clone()
            },
            None => {
                let content = ContentUnit::System(
                    "The following is a conversation with an AI Robot. The Robot is helpful, creative, clever, and very friendly. ".to_string());
                self.add_content(room_id, content.clone());
                vec![content]
            }
        }
    }

    pub fn clear_content(&mut self, room_id:&String) {
        self.contents.remove(room_id);
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

    pub fn remove_client(&mut self, client:&ClientUnit) {
        self.data.remove(&(client.addr));
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_unit_migrate_content_test() {
        let mut cu = ClientUnit::new("".to_string());
        cu.add_content(&("1".to_string()), ContentUnit::Human("hihihi".to_string()));
        cu.add_content(&("1".to_string()), ContentUnit::Robot("hi".to_string()));
        assert_eq!(cu.migrate_content(&("1".to_string())), vec![
            ContentUnit::Human("hihihi".to_string()),
            ContentUnit::Robot("hi".to_string())
        ]);
    }
}
