extern crate serde_json;

use self::serde_json::Value as JsonValue;

#[derive(Debug, Clone)]
pub struct Event {
    pub sender: String,
    pub stype: String,
    pub room: String,
    pub id: String,
    pub content: JsonValue,
}
