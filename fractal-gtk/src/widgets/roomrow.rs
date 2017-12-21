extern crate url;
extern crate gtk;
extern crate gdk_pixbuf;

use self::url::Url;
use self::gdk_pixbuf::Pixbuf;
use self::gtk::prelude::*;

use fractal_api;
use types::Room;

use util::glib_thread_prelude::*;

// Room row for the room sidebar. This widget shows the room avatar, the room name and the unread
// messages in the room
// +-----+--------------------------+------+
// | IMG | Fractal                  |  32  |
// +-----+--------------------------+------+
pub struct RoomRow {
    pub room: Room,
    pub icon: gtk::Image,
    pub text: gtk::Label,
    pub notifications: gtk::Label,
}

impl RoomRow {
    pub fn new(room: Room, baseu: &Url) -> RoomRow {
        let name = room.name.clone().unwrap_or_default();
        let avatar = room.avatar.clone().unwrap_or_default();
        let icon = gtk::Image::new();
        let text = gtk::Label::new(name.as_str());
        let notifications = gtk::Label::new(&format!("{}", room.notifications)[..]);

        default_avatar(&icon);
        download_avatar(baseu, avatar, &icon);

        RoomRow {
            room,
            icon,
            text,
            notifications,
        }
    }

    pub fn widget(&self) -> gtk::Box {
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        b.pack_start(&self.icon, false, false, 5);
        b.pack_start(&self.text, true, true, 5);
        b.pack_start(&self.notifications, false, false, 5);
        b.show_all();

        b
    }
}

fn download_avatar(baseu: &Url, avatar: String, image: &gtk::Image) {
    let url = baseu.clone();
    let img = image.clone();
    glib_thread!(Result<String, Error>,
        || {
            if let Ok(i) = fractal_api::util::dw_media(&url, &avatar, true, None, 40, 40) {
                return fractal_api::util::circle_image(i);
            }

            return Err(Error::BackendError);
        },
        |rc: Result<String, Error>| {
            if let Ok(c) = rc {
                if let Ok(pixbuf) = Pixbuf::new_from_file_at_scale(&c, 40, 40, false) {
                    img.set_from_pixbuf(&pixbuf);
                }
            }
        }
    );
}

fn default_avatar(img: &gtk::Image) {
    img.set_from_icon_name("avatar-default-symbolic", 5);
}
