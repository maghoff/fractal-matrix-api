pub use model::event::Event;
pub use model::room::Room;
pub use model::protocol::Protocol;
pub use model::message::Message;
pub use model::member::Member;
pub use model::member::MemberList;

use std::collections::HashMap;
use std::time::Instant;

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
