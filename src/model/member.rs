use std::collections::HashMap;

#[derive(Debug)]
pub struct Member {
    pub alias: String,
    pub uid: String,
    pub avatar: String,
}

impl Member {
    pub fn get_alias(&self) -> String {
        match self.alias {
            ref a if a.is_empty() => self.uid.clone(),
            ref a => a.clone(),
        }
    }
}

// hashmap userid -> Member
pub type MemberList = HashMap<String, Member>;
