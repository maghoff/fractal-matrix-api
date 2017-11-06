extern crate xdg;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

use types::RoomList;
use error::Error;

use std::collections::HashMap;
use std::time::Instant;

use util::cache_path;

pub struct CacheMap<T> {
    map: HashMap<String, (Instant, T)>,
    timeout: u64,
}

impl<T> CacheMap<T> {
    pub fn new() -> CacheMap<T> {
        CacheMap { map: HashMap::new(), timeout: 10 }
    }

    pub fn timeout(mut self, timeout: u64) -> CacheMap<T> {
        self.timeout = timeout;
        self
    }

    pub fn get(&self, k: &String) -> Option<&T> {
        match self.map.get(k) {
            Some(t) => {
                if t.0.elapsed().as_secs() >= self.timeout {
                    return None;
                }
                Some(&t.1)
            }
            None => None
        }
    }

    pub fn insert(&mut self, k: String, v: T) {
        let now = Instant::now();
        self.map.insert(k, (now, v));
    }
}


#[derive(Serialize, Deserialize)]
pub struct CacheData {
    pub since: String,
    pub rooms: RoomList,
    pub username: String,
    pub uid: String,
}


pub fn store(rooms: &RoomList, since: String, username: String, uid: String) -> Result<(), Error> {
    let fname = cache_path("rooms.json")?;

    let data = CacheData {
        since: since,
        rooms: rooms.clone(),
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
