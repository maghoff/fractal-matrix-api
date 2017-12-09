use std::collections::HashMap;
use model::message::Message;
use model::member::MemberList;

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub avatar: Option<String>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub alias: Option<String>,
    pub guest_can_join: bool,
    pub world_readable: bool,
    pub n_members: i32,
    pub members: MemberList,
    pub notifications: i32,
    pub messages: Vec<Message>,
}

impl Room {
    pub fn new(id: String, name: Option<String>) -> Room {
        Room {
            id: id,
            name: name,
            avatar: None,
            topic: None,
            alias: None,
            guest_can_join: true,
            world_readable: true,
            n_members: 0,
            notifications: 0,
            messages: vec![],
            members: HashMap::new(),
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
            n_members: self.n_members,
            notifications: self.notifications,
            messages: self.messages.iter().cloned().collect(),
            members: self.members.clone(),
        }
    }
}

pub type RoomList = HashMap<String, Room>;
