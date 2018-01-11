extern crate url;
extern crate gtk;
extern crate pango;

use glib;

use self::url::Url;
use std::collections::HashMap;
use self::gtk::prelude::*;

use widgets::roomrow::RoomRow;
use types::Room;
use types::Message;
use std::sync::{Arc, Mutex};


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


pub struct RoomListGroup {
    pub rooms: HashMap<String, RoomRow>,
    pub baseu: Url,
    list: gtk::ListBox,
    rev: gtk::Revealer,
    arrow: gtk::Arrow,
    title: gtk::Label,
    empty: gtk::Label,
    title_eb: gtk::EventBox,
    widget: gtk::Box,

    roomvec: Arc<Mutex<Vec<Room>>>,
    // TODO:
    // * Add drag & drop support for favorites
}

impl RoomListGroup {
    pub fn new(url: &Url, name: &str, empty_text: &str) -> RoomListGroup {
        let list = gtk::ListBox::new();
        let baseu = url.clone();
        let rooms = HashMap::new();
        let roomvec = Arc::new(Mutex::new(vec![]));

        let empty = gtk::Label::new(empty_text);
        empty.set_line_wrap_mode(pango::WrapMode::WordChar);
        empty.set_line_wrap(true);
        empty.set_justify(gtk::Justification::Center);
        if let Some(style) = empty.get_style_context() {
            style.add_class("room-empty-text");
        }

        let rev = gtk::Revealer::new();
        let b = gtk::Box::new(gtk::Orientation::Vertical, 0);
        b.add(&empty);
        b.add(&list);

        rev.add(&b);
        rev.set_reveal_child(true);

        let title = gtk::Label::new(name);
        title.set_alignment(0.0, 0.0);
        let arrow = gtk::Arrow::new(gtk::ArrowType::Down, gtk::ShadowType::None);
        let title_eb = gtk::EventBox::new();

        let a = arrow.clone();
        let r = rev.clone();
        title_eb.connect_button_press_event(move |_, _| {
            match a.get_property_arrow_type() {
                gtk::ArrowType::Down => {
                    a.set(gtk::ArrowType::Up, gtk::ShadowType::None);
                    r.set_reveal_child(false);
                }
                _ => {
                    a.set(gtk::ArrowType::Down, gtk::ShadowType::None);
                    r.set_reveal_child(true);
                }
            };
            glib::signal::Inhibit(true)
        });

        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);

        RoomListGroup {
            list,
            baseu,
            rooms,
            roomvec,
            rev,
            title,
            arrow,
            title_eb,
            widget,
            empty,
        }
    }

    pub fn add_room(&mut self, r: Room) {
        if self.rooms.contains_key(&r.id) {
            // room added, we'll pass
            return;
        }

        let rid = r.id.clone();
        self.roomvec.lock().unwrap().push(r.clone());

        let row = RoomRow::new(r, &self.baseu);
        self.list.add(&row.widget());

        self.rooms.insert(rid, row);
    }

    pub fn prepend_room(&mut self, r: Room) {
        if self.rooms.contains_key(&r.id) {
            // room added, we'll pass
            return;
        }

        let rid = r.id.clone();
        self.roomvec.lock().unwrap().insert(0, r.clone());

        let row = RoomRow::new(r, &self.baseu);
        self.list.prepend(&row.widget());

        self.rooms.insert(rid, row);
    }

    pub fn set_room_notifications(&mut self, room: String, n: i32) {
        if let Some(r) = self.rooms.get(&room) {
            r.set_notifications(n);
        }

        self.edit_room(&room, move |rv| { rv.notifications = n; });
    }

    pub fn remove_room(&mut self, room: String) -> Option<Room> {
        self.rooms.remove(&room);
        let mut rv = self.roomvec.lock().unwrap();
        if let Some(idx) = rv.iter().position(|x| { x.id == room}) {
            if let Some(row) = self.list.get_row_at_index(idx as i32) {
                self.list.remove(&row);
            }
            return Some(rv.remove(idx));
        }

        None
    }

    pub fn rename_room(&mut self, room: String, newname: Option<String>) {
        if let (Some(r), Some(n)) = (self.rooms.get_mut(&room), newname.clone()) {
            r.set_name(n);
        }

        self.edit_room(&room, move |rv| { rv.name = newname.clone(); });
    }

    pub fn set_room_avatar(&mut self, room: String, av: Option<String>) {
        if let Some(r) = self.rooms.get_mut(&room) {
            r.set_avatar(av.clone());
        }

        self.edit_room(&room, move |rv| { rv.avatar = av.clone(); });
    }

    pub fn widget(&self) -> gtk::Box {
        let b = self.widget.clone();
        if let Some(style) = b.get_style_context() {
            style.add_class("room-list");
        }

        // building the heading
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        if let Some(style) = hbox.get_style_context() {
            style.add_class("room-title");
        }
        hbox.pack_start(&self.title, true, true, 0);
        hbox.pack_start(&self.arrow, false, false, 0);

        for ch in self.title_eb.get_children() {
            self.title_eb.remove(&ch);
        }
        self.title_eb.add(&hbox);

        self.arrow.set(gtk::ArrowType::Down, gtk::ShadowType::None);
        self.rev.set_reveal_child(true);
        b.pack_start(&self.title_eb, false, false, 0);
        b.pack_start(&self.rev, true, true, 0);

        self.show();

        b
    }

    pub fn show(&self) {
        self.widget.show_all();
        self.render_notifies();
        if self.rooms.is_empty() {
            self.empty.show();
            self.list.hide();
        } else {
            self.list.show();
            self.empty.hide();
        }
    }

    pub fn hide(&self) {
        self.widget.hide();
    }

    pub fn connect<F: Fn(Room) + 'static>(&self, cb: F) {
        let rs = self.roomvec.clone();
        self.list.connect_row_activated(move |_, row| {
            let idx = row.get_index();
            cb(rs.lock().unwrap()[idx as usize].clone());
        });
    }

    pub fn get_selected(&self) -> Option<String> {
        let rv = self.roomvec.lock().unwrap();
        match self.list.get_selected_row() {
            Some(row) => Some(rv[row.get_index() as usize].id.clone()),
            None => None,
        }
    }

    pub fn set_selected(&self, room: Option<String>) {
        self.list.unselect_all();

        if room.is_none() {
            return;
        }

        let room = room.unwrap();

        let rv = self.roomvec.lock().unwrap();
        if let Some(idx) = rv.iter().position(|x| { x.id == room}) {
            if let Some(ref row) = self.list.get_row_at_index(idx as i32) {
                self.list.select_row(row);
            }
        }
    }

    pub fn add_rooms(&mut self, mut array: Vec<Room>) {
        array.sort_by_key(|ref x| {
            match x.messages.last() {
                Some(l) => l.date,
                None => Message::default().date,
            }
        });

        for r in array.iter().rev() {
            self.add_room(r.clone());
        }
    }

    pub fn moveup(&mut self, room: String) {
        let s = self.get_selected();

        if let Some(r) = self.remove_room(room) {
            self.prepend_room(r);
        }

        self.set_selected(s);
    }

    fn render_notifies(&self) {
        for (_k, r) in self.rooms.iter() {
            r.render_notifies();
        }
    }

    fn edit_room<F: Fn(&mut Room) + 'static>(&mut self, room: &str, cb: F) {
        let mut rv = self.roomvec.lock().unwrap();
        if let Some(idx) = rv.iter().position(|x| { x.id == room}) {
            if let Some(ref mut m) = rv.get_mut(idx) {
                cb(m);
            }
        }
    }
}


pub struct RoomList {
    pub baseu: Url,
    widget: gtk::Box,

    inv: RoomListGroup,
    fav: RoomListGroup,
    rooms: RoomListGroup,
}

macro_rules! run_in_group {
    ($self: expr, $roomid: expr, $fn: ident, $($arg: expr),*) => {{
        if $self.inv.rooms.contains_key($roomid) {
            $self.inv.$fn($($arg),*)
        } else if $self.fav.rooms.contains_key($roomid) {
            $self.fav.$fn($($arg),*)
        } else {
            $self.rooms.$fn($($arg),*)
        }
    }}
}

impl RoomList {
    pub fn new(url: Option<String>) -> RoomList {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let baseu = get_url(url);

        let inv = RoomListGroup::new(&baseu, "Invites", "You don't have any invitations");
        let fav = RoomListGroup::new(&baseu, "Favorites", "Drag and drop rooms here to \
                                                           add them to your favorites");
        let rooms = RoomListGroup::new(&baseu, "Rooms", "You don't have any rooms yet");

        RoomList {
            baseu,
            widget,
            inv,
            fav,
            rooms,
        }
    }

    pub fn set_selected(&self, room: Option<String>) {
        self.inv.set_selected(None);
        self.fav.set_selected(None);
        self.rooms.set_selected(None);

        if let Some(r) = room {
            run_in_group!(self, &r, set_selected, Some(r.clone()));
        }
    }

    pub fn get_selected(&self) -> Option<String> {
        for i in [&self.inv, &self.fav, &self.rooms].iter() {
            if let Some(s) = i.get_selected() {
                return Some(s.clone());
            }
        }
        None
    }

    pub fn add_rooms(&mut self, array: Vec<Room>) {
        //TODO split between favs and invites
        self.rooms.add_rooms(array);
        self.show_and_hide();
    }

    pub fn connect<F: Fn(Room) + 'static>(&self, cb: F) {
        let acb = Arc::new(cb);

        let cb = acb.clone();
        self.inv.connect(move |room| cb(room));
        let cb = acb.clone();
        self.fav.connect(move |room| cb(room));
        let cb = acb.clone();
        self.rooms.connect(move |room| cb(room));
    }

    pub fn set_room_avatar(&mut self, room: String, av: Option<String>) {
        run_in_group!(self, &room, set_room_avatar, room, av);
    }

    pub fn set_room_notifications(&mut self, room: String, n: i32) {
        run_in_group!(self, &room, set_room_notifications, room, n);
    }

    pub fn remove_room(&mut self, room: String) -> Option<Room> {
        run_in_group!(self, &room, remove_room, room)
    }

    pub fn add_room(&mut self, r: Room) {
        // TODO add to the corresponding group
        self.rooms.add_room(r);
        self.show_and_hide();
    }

    pub fn rename_room(&mut self, room: String, newname: Option<String>) {
        run_in_group!(self, &room, rename_room, room, newname);
    }

    pub fn moveup(&mut self, room: String) {
        run_in_group!(self, &room, moveup, room);
    }

    pub fn widget(&self) -> gtk::Box {
        for ch in self.widget.get_children() {
            self.widget.remove(&ch);
        }
        self.widget.add(&self.inv.widget());
        self.widget.add(&self.fav.widget());
        self.widget.add(&self.rooms.widget());

        self.show_and_hide();

        self.widget.clone()
    }

    pub fn show_and_hide(&self) {
        self.widget.show_all();

        if self.inv.rooms.is_empty() {
            self.inv.hide();
        } else {
            self.inv.show();
        }

        self.fav.show();
        self.rooms.show();
    }
}
