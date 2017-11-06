extern crate chrono;

use self::chrono::prelude::*;

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub mtype: String,
    pub body: String,
    pub date: DateTime<Local>,
    pub room: String,
    pub thumb: String,
    pub url: String,
    pub id: String,
}

impl Clone for Message {
    fn clone(&self) -> Message {
        Message {
            sender: self.sender.clone(),
            mtype: self.mtype.clone(),
            body: self.body.clone(),
            date: self.date.clone(),
            room: self.room.clone(),
            thumb: self.thumb.clone(),
            url: self.url.clone(),
            id: self.id.clone(),
        }
    }
}
