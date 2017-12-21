extern crate url;
extern crate gtk;

use self::url::Url;
use std::collections::HashMap;
use self::gtk::prelude::*;

use widgets::roomrow::RoomRow;
use types::Room;

pub struct RoomList {
    pub rooms: HashMap<String, RoomRow>,
    pub baseu: Url,
    list: gtk::ListBox,
    // TODO:
    // * Add a header to the list
    // * Add a collapse/expand button with a revealer
    // * Add drag & drop support for favorites
}

impl RoomList {
    pub fn new(url: Option<Url>) -> RoomList {
        let list = gtk::ListBox::new();
        let baseu = match url {
            Some(u) => u.clone(),
            None => Url::parse("https://matrix.org").unwrap()
        };
        let rooms = HashMap::new();

        RoomList {
            list,
            baseu,
            rooms,
        }
    }

    pub fn add_room(&mut self, r: Room) {
        if self.rooms.contains_key(&r.id) {
            // room added, we'll pass
            return;
        }

        let rid = r.id.clone();
        let row = RoomRow::new(r, &self.baseu);
        self.list.add(&row.widget());

        self.rooms.insert(rid, row);
    }

    pub fn set_room_notifications(&self, room: String, n: i32) {
        if let Some(r) = self.rooms.get(&room) {
            r.notifications.set_text(&format!("{}", n));
        }
    }

    pub fn remove_room(&mut self, room: String) {
        // TODO: implement this...
    }

    pub fn rename_room(&self, room: String, newname: Option<String>) {
        // TODO: implement this...
    }

    pub fn avatar_room(&self, room: Option<String>) {
        // TODO: implement this...
    }

    pub fn widget(&self) -> gtk::Box {
        let b = gtk::Box::new(gtk::Orientation::Vertical, 0);

        b.pack_start(&self.list, true, true, 0);
        b.show_all();

        b
    }
}
