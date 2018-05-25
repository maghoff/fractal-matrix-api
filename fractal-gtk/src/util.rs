extern crate glib;
extern crate cairo;
extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate tree_magic;

use self::gtk::ImageExt;
use self::gtk::IconThemeExt;
use self::gtk::WidgetExt;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::TryRecvError;
use std::path::Path;
use backend::BKCommand;
use self::gdk_pixbuf::Pixbuf;
use self::gdk_pixbuf::PixbufExt;
use self::gdk_pixbuf::PixbufAnimation;
use self::gdk_pixbuf::PixbufAnimationExt;
use failure::Error;
use self::gdk::ContextExt;

use html2pango::{html_escape, markup_links};

pub mod glib_thread_prelude {
    pub use std::thread;
    pub use std::sync::mpsc::channel;
    pub use std::sync::mpsc::{Sender, Receiver};
    pub use std::sync::mpsc::TryRecvError;
    pub use error::Error;
}


#[macro_export]
macro_rules! glib_thread {
    ($type: ty, $thread: expr, $glib_code: expr) => {{
        let (tx, rx): (Sender<$type>, Receiver<$type>) = channel();
        thread::spawn(move || {
            let output = $thread();
            tx.send(output).unwrap();
        });

        gtk::timeout_add(50, move || match rx.try_recv() {
            Err(TryRecvError::Empty) => gtk::Continue(true),
            Err(TryRecvError::Disconnected) => {
                eprintln!("glib_thread error");
                gtk::Continue(false)
            }
            Ok(output) => {
                $glib_code(output);
                gtk::Continue(false)
            }
        });
    }}
}

pub fn get_pixbuf_data(pb: &Pixbuf) -> Result<Vec<u8>, Error> {
    let image = cairo::ImageSurface::create(cairo::Format::ARgb32,
                                            pb.get_width(),
                                            pb.get_height())
        .or(Err(format_err!("Cairo Error")))?;

    let g = cairo::Context::new(&image);
    g.set_source_pixbuf(pb, 0.0, 0.0);
    g.paint();

    let mut buf: Vec<u8> = Vec::new();
    image.write_to_png(&mut buf)?;
    Ok(buf)
}

pub fn markup_text(s: &str) -> String {
    markup_links(&html_escape(s))
}

pub struct Thumb(pub bool);

/// If `path` starts with mxc this func download the img async, in other case the image is loaded
/// in the `image` widget scaled to size
pub fn load_async(backend: &Sender<BKCommand>,
                  path: &str,
                  img: &gtk::Image,
                  size: (i32, i32),
                  Thumb(thumb): Thumb) {

    if path.starts_with("mxc:") {
        let pixbuf = match gtk::IconTheme::get_default() {
            None => None,
            Some(i1) => match i1.load_icon("image-loading-symbolic", size.0, gtk::IconLookupFlags::empty()) {
                Err(_) => None,
                Ok(i2) => i2,
            }
        };
        if let Some(pix) = pixbuf {
            img.set_from_pixbuf(&pix);
        }

        // asyn load
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        let command = match thumb {
            false => BKCommand::GetMediaAsync(path.to_string(), tx),
            true => BKCommand::GetThumbAsync(path.to_string(), tx),
        };
        backend.send(command).unwrap();
        let im = img.clone();
        gtk::timeout_add(50, move || match rx.try_recv() {
            Err(TryRecvError::Empty) => gtk::Continue(true),
            Err(TryRecvError::Disconnected) => gtk::Continue(false),
            Ok(fname) => {
                load_pixbuf(&fname, &im, size);
                gtk::Continue(false)
            }
        });
    } else {
        load_pixbuf(path, &img, size);
    }
}

pub fn is_gif(fname: &str) -> bool {
    let result = tree_magic::from_filepath(&Path::new(fname));
    result == "image/gif"
}

pub fn load_pixbuf(fname: &str, image: &gtk::Image, size: (i32, i32)) {
    if is_gif(&fname) {
        load_animation(&fname, &image, size);
        return;
    }

    let pixbuf = Pixbuf::new_from_file(fname);
    if let Ok(pix) = pixbuf {
        if let Err(_) = draw_pixbuf(&pix, image, size) {
          image.set_from_file(fname);
        }
    } else {
        image.set_from_file(fname);
    }
}

pub fn draw_pixbuf(pix: &Pixbuf, image: &gtk::Image, size: (i32, i32)) -> Result<(), ()> {
    let mut w = pix.get_width();
    let mut h = pix.get_height();

    if w > h && w > size.0 {
        h = size.1 * h / w;
        w = size.0;
    } else if h >= w && h > size.1 {
        w = size.0 * w / h;
        h = size.1;
    }

    if let Some(scaled) = pix.scale_simple(w, h, gdk_pixbuf::InterpType::Bilinear) {
        image.set_from_pixbuf(&scaled);
        return Ok(())
    }

    Err(())
}

pub fn load_animation(fname: &str, image: &gtk::Image, size: (i32, i32)) {
    let res = PixbufAnimation::new_from_file(fname);
    if res.is_err() {
        return;
    }
    let anim = res.unwrap();
    let iter = anim.get_iter(&glib::get_current_time());

    let im = image.clone();
    gtk::timeout_add(iter.get_delay_time() as u32, move || {
        iter.advance(&glib::get_current_time());

        if im.is_drawable() {
            let pix = iter.get_pixbuf();
            let _ = draw_pixbuf(&pix, &im, size);
        } else {
            if let None = im.get_parent() {
                return gtk::Continue(false);
            }
        }
        gtk::Continue(true)
    });
}
