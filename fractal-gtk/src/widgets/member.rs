extern crate gtk;
extern crate gdk_pixbuf;

use self::gdk_pixbuf::Pixbuf;
use self::gtk::prelude::*;

use types::Member;

use backend::BKCommand;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

use app::AppOp;

// Room Search item
pub struct MemberBox<'a> {
    member: &'a Member,
    op: &'a AppOp,
}

impl<'a> MemberBox<'a> {
    pub fn new(member: &'a Member, op: &'a AppOp) -> MemberBox<'a> {
        MemberBox {
            member: member,
            op: op,
        }
    }

    pub fn widget(&self) -> gtk::EventBox {
        let backend = self.op.backend.clone();
        let username = gtk::Label::new("");
        let event_box = gtk::EventBox::new();
        let w = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        username.set_text(&self.member.get_alias().unwrap_or_default());

        let avatar = gtk::Image::new_from_icon_name("avatar-default-symbolic", 3);
        get_member_avatar(backend.clone(), avatar.clone(), Some(self.member.clone()), 30, 3);
        avatar.set_alignment(0.5, 0.);

        w.pack_start(&avatar, false, false, 5);
        w.pack_start(&username, false, false, 5);

        event_box.add(&w);
        event_box.show_all();
        event_box
    }
}

pub fn get_member_avatar(backend: Sender<BKCommand>, img: gtk::Image, m: Option<Member>, size: i32, tries: i32) {
    if tries <= 0 {
        return;
    }

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    backend.send(BKCommand::GetAvatarAsync(m.clone(), tx)).unwrap();
    gtk::timeout_add(50, move || match rx.try_recv() {
        Err(_) => gtk::Continue(true),
        Ok(avatar) => {
            if let Ok(pixbuf) = Pixbuf::new_from_file_at_scale(&avatar, size, size, false) {
                img.set_from_pixbuf(&pixbuf);
            } else {
                // trying again if fail
                img.set_from_icon_name("avatar-default-symbolic", 5);
                get_member_avatar(backend.clone(), img.clone(), m.clone(), size, tries - 1);
            }

            gtk::Continue(false)
        }
    });
}
