use std::collections::HashMap;

#[derive(Debug)]
pub struct Member {
    pub alias: Option<String>,
    pub uid: String,
    pub avatar: Option<String>,
}

impl Member {
    pub fn get_alias(&self) -> Option<String> {
        match self.alias {
            ref a if a.is_none() || a.clone().unwrap().is_empty() => Some(self.uid.clone()),
            ref a => a.clone(),
        }
    }
}

// hashmap userid -> Member
pub type MemberList = HashMap<String, Member>;
