extern crate url;
extern crate gtk;

use self::url::Url;
use std::collections::HashMap;
use self::gtk::prelude::*;

use widgets::roomrow::RoomRow;
use types::Room;


fn get_url(url: Option<String>) -> Url {
    let defurl = Url::parse("https://matrix.org").unwrap();

    match url {
        Some(u) => {
            match Url::parse(&u) {
                Ok(url) => url,
                Err(_) => defurl,
            }
        }
        None => defurl,
    }
}


pub struct RoomList {
    pub rooms: HashMap<String, RoomRow>,
    pub baseu: Url,
    list: gtk::ListBox,

    roomvec: Vec<Room>,
    // TODO:
    // * Add a header to the list
    // * Add a collapse/expand button with a revealer
    // * Add drag & drop support for favorites
}

impl RoomList {
    pub fn new(url: Option<String>) -> RoomList {
        let list = gtk::ListBox::new();
        let baseu = get_url(url);
        let rooms = HashMap::new();
        let roomvec = vec![];

        RoomList {
            list,
            baseu,
            rooms,
            roomvec,
        }
    }

    pub fn add_room(&mut self, r: Room) {
        if self.rooms.contains_key(&r.id) {
            // room added, we'll pass
            return;
        }

        let rid = r.id.clone();
        self.roomvec.push(r.clone());

        let row = RoomRow::new(r, &self.baseu);
        self.list.add(&row.widget());

        self.rooms.insert(rid, row);
    }

    pub fn set_room_notifications(&self, room: String, n: i32) {
        if let Some(r) = self.rooms.get(&room) {
            r.set_notifications(n);
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
        if let Some(style) = b.get_style_context() {
            style.add_class("room-list");
        }

        b.pack_start(&self.list, true, true, 0);
        b.show_all();

        b
    }

    pub fn connect<F: Fn(Room) + 'static>(&self, cb: F) {
        let rs = self.roomvec.clone();
        self.list.connect_row_activated(move |_, row| {
            let idx = row.get_index();
            cb(rs[idx as usize].clone());
        });
    }
}
