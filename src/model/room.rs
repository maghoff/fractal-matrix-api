use std::collections::HashMap;
use model::message::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub avatar: String,
    pub name: String,
    pub topic: String,
    pub alias: String,
    pub guest_can_join: bool,
    pub world_readable: bool,
    pub members: i32,
    pub notifications: i32,
    pub messages: Vec<Message>,
}

impl Room {
    pub fn new(id: String, name: String) -> Room {
        Room {
            id: id,
            name: name,
            avatar: String::new(),
            topic: String::new(),
            alias: String::new(),
            guest_can_join: true,
            world_readable: true,
            members: 0,
            notifications: 0,
            messages: vec![],
        }
    }
}

impl Clone for Room {
    fn clone(&self) -> Room {
        Room {
            id: self.id.clone(),
            name: self.name.clone(),
            avatar: self.avatar.clone(),
            topic: self.topic.clone(),
            alias: self.alias.clone(),
            guest_can_join: self.guest_can_join,
            world_readable: self.world_readable,
            members: self.members,
            notifications: self.notifications,
            messages: self.messages.iter().cloned().collect(),
        }
    }
}

pub type RoomList = HashMap<String, Room>;
