extern crate pango;
extern crate url;
extern crate gdk;
extern crate gtk;
extern crate cairo;

use self::url::Url;
use self::gtk::prelude::*;

use fractal_api;
use fractal_api::util::AvatarMode;
use fractal_api::util::draw_identicon;

use types::Room;

use util::glib_thread_prelude::*;

use widgets;
use widgets::AvatarExt;


const ICON_SIZE: i32 = 20;


// Room row for the room sidebar. This widget shows the room avatar, the room name and the unread
// messages in the room
// +-----+--------------------------+------+
// | IMG | Fractal                  |  32  |
// +-----+--------------------------+------+
pub struct RoomRow {
    baseu: Url,
    pub room: Room,
    pub icon: widgets::Avatar,
    pub text: gtk::Label,
    pub notifications: gtk::Label,
    pub widget: gtk::EventBox,
}

impl RoomRow {
    pub fn new(room: Room, url: &Url) -> RoomRow {
        let widget = gtk::EventBox::new();
        let name = room.name.clone().unwrap_or_default();
        let avatar = room.avatar.clone().unwrap_or_default();
        let icon = widgets::Avatar::avatar_new(Some(ICON_SIZE));
        let text = gtk::Label::new(name.clone().as_str());
        let baseu = url.clone();
        text.set_alignment(0.0, 0.0);
        text.set_ellipsize(pango::EllipsizeMode::End);

        let n = room.notifications;
        let notifications = gtk::Label::new(&format!("{}", n)[..]);
        if let Some(style) = notifications.get_style_context() {
            style.add_class("notify-badge");
        }
        match n {
            0 => notifications.hide(),
            _ => notifications.show(),
        }

        icon.default(String::from("avatar-default-symbolic"), Some(ICON_SIZE));
        if avatar.starts_with("mxc") || avatar.is_empty() {
            download_avatar(&baseu, room.id.clone(), name, avatar, &icon);
        } else {
            icon.circle(avatar, Some(ICON_SIZE));
        }

        let rr = RoomRow {
            room,
            icon,
            text,
            notifications,
            baseu,
            widget,
        };

        rr.connect_dnd();

        rr
    }

    pub fn set_notifications(&self, n: i32) {
        self.notifications.set_text(&format!("{}", n));
        match n {
            0 => self.notifications.hide(),
            _ => self.notifications.show(),
        }
    }

    pub fn render_notifies(&self) {
        match self.room.notifications {
            0 => self.notifications.hide(),
            _ => self.notifications.show(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.room.name = Some(name.clone());
        self.text.set_text(&name);
    }

    pub fn set_avatar(&mut self, avatar: Option<String>) {
        let name = self.room.name.clone().unwrap_or_default();
        self.room.avatar = avatar.clone();

        self.icon.default(String::from("avatar-default-symbolic"), Some(ICON_SIZE));
        let av = avatar.unwrap_or_default();
        if av.starts_with("mxc") || av.is_empty() {
            download_avatar(&self.baseu, self.room.id.clone(), name, av, &self.icon);
        } else {
            self.icon.circle(av, Some(ICON_SIZE));
        }
    }

    pub fn widget(&self) -> gtk::EventBox {
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        for ch in self.widget.get_children() {
            self.widget.remove(&ch);
        }
        self.widget.add(&b);

        if let Some(style) = b.get_style_context() {
            style.add_class("room-row");
        }

        b.pack_start(&self.icon, false, false, 5);
        b.pack_start(&self.text, true, true, 0);
        b.pack_start(&self.notifications, false, false, 5);
        self.widget.show_all();

        if self.room.notifications == 0 {
            self.notifications.hide();
        }

        self.widget.clone()
    }

    pub fn connect_dnd(&self) {
        if self.room.inv {
            return;
        }

        let mask = gdk::ModifierType::BUTTON1_MASK;
        let actions = gdk::DragAction::MOVE;
        self.widget.drag_source_set(mask, &[], actions);
        self.widget.drag_source_add_text_targets();

        self.widget.connect_drag_begin(move |w, ctx| {
            let ww = w.get_allocated_width();
            let wh = w.get_allocated_height();
            let image = cairo::ImageSurface::create(cairo::Format::ARgb32, ww, wh).unwrap();
            let g = cairo::Context::new(&image);
            g.set_source_rgba(1.0, 1.0, 1.0, 0.8);
            g.rectangle(0.0, 0.0, ww as f64, wh as f64);
            g.fill();

            w.draw(&g);

            ctx.drag_set_icon_surface(&image);
        });

        let id = self.room.id.clone();
        self.widget.connect_drag_data_get(move |_w, _, data, _x, _y| {
            data.set_text(&id);
        });
    }
}

fn download_avatar(baseu: &Url,
                   rid: String,
                   name: String,
                   avatar: String,
                   image: &widgets::Avatar) {

    let url = baseu.clone();
    let img = image.clone();
    glib_thread!(Result<String, Error>,
        || {
            match avatar {
                ref s if s.is_empty() => identicon!(&rid, name),
                _ => fractal_api::util::dw_media(&url, &avatar, true, None, 40, 40),
            }
        },
        |rc: Result<String, Error>| {
            if let Ok(c) = rc {
                img.circle(c, Some(ICON_SIZE));
            }
        }
    );
}
