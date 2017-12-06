extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

use types::RoomList;
use error::Error;

use fractal_api::util::cache_path;
use globals;

#[derive(Serialize, Deserialize)]
pub struct CacheData {
    pub since: String,
    pub rooms: RoomList,
    pub username: String,
    pub uid: String,
}


pub fn store(rooms: &RoomList, since: String, username: String, uid: String) -> Result<(), Error> {
    let fname = cache_path("rooms.json")?;

    let mut cacherooms = rooms.clone();
    for r in cacherooms.values_mut() {
        let skip = match r.messages.len() {
            n if n > globals::CACHE_SIZE => n - globals::CACHE_SIZE,
            _ => 0,
        };
        r.messages = r.messages.iter().skip(skip).cloned().collect();
    }

    let data = CacheData {
        since: since,
        rooms: cacherooms,
        username: username,
        uid: uid,
    };

    let serialized = serde_json::to_string(&data)?;
    File::create(fname)?.write_all(&serialized.into_bytes())?;

    Ok(())
}

pub fn load() -> Result<CacheData, Error> {
    let fname = cache_path("rooms.json")?;

    let mut file = File::open(fname)?;
    let mut serialized = String::new();
    file.read_to_string(&mut serialized)?;

   let deserialized: CacheData = serde_json::from_str(&serialized)?;

   Ok(deserialized)
}
