use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};


use simple_openai::ContentType;

#[derive(Debug, Clone)]
pub struct ClientUnit {
    addr: String,
    contents: HashMap<String, Vec<ContentType>>,
}

impl ClientUnit {
    pub fn new(addr:String) -> Self {
        Self { 
            addr,
            contents: HashMap::new()
        }
    }

    pub fn add_content(&mut self, room_id: &String, content:ContentType) {
        match self.contents.get_mut(room_id) {
            Some(c) => {
                c.push(content);
                if c.len() >=10 {
                    for _ in 0..6 {
                        c.remove(1);
                    }
                }
            },
            None => {
                let c_l = vec![content];
                self.contents.insert(room_id.clone(), c_l);
            }
        }

    }

    pub fn migrate_content(&mut self, room_id: &String) -> Vec<ContentType> {
        match self.contents.get(room_id).as_mut() {
            Some(&mut x) => {
                return x.clone()
            },
            None => {
                let content = ContentType::system(
                    //"The following is a conversation with an AI Robot. The Robot is helpful, creative, clever, and very friendly. ".to_string());
                    "你是一个17岁傲娇软萌聪明活泼的女高中生。".to_string());
                self.add_content(room_id, content.clone());
                vec![content]
            }
        }
    }

    pub fn clear_content(&mut self, room_id:&String) {
        self.contents.remove(room_id);
    }

    pub fn rooms(&mut self) -> Vec<&String>{
        return self.contents.keys().collect();
        
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

    pub fn remove_client(&mut self, addr:String) {
        self.data.remove(&addr);
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_unit_migrate_content_test() {
        let mut cu = ClientUnit::new("".to_string());
        cu.add_content(&("1".to_string()), ContentType::user("hihihi".to_string()));
        cu.add_content(&("1".to_string()), ContentType::assistant("hi".to_string()));
        assert_eq!(cu.migrate_content(&("1".to_string())), vec![
            ContentType::user("hihihi".to_string()),
            ContentType::assistant("hi".to_string())
        ]);
    }

    #[test]
    fn add_content_max_test() {
        let mut cu = ClientUnit::new("".to_string());
        let room_id = "1".to_string();
        for i in 0..30 {
            cu.add_content(&room_id, ContentType::user("hihihi".to_string()));
        }
        assert!(
            cu.migrate_content(&room_id).len()<10
        );
        
    }
}
