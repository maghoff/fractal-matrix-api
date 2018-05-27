extern crate gtk;
extern crate glib;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate tree_magic;

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use self::gtk::prelude::*;
use self::gtk::DrawingArea;
use self::gdk_pixbuf::Pixbuf;
use self::gdk_pixbuf::PixbufExt;
use self::gdk::ContextExt;
use self::gdk_pixbuf::PixbufAnimation;
use self::gdk_pixbuf::PixbufAnimationExt;

use backend::BKCommand;
use std::sync::mpsc::TryRecvError;

pub struct Thumb(pub bool);

#[derive(Clone, Debug)]
pub struct Image {
    pub path: String,
    pub max_size: (i32, i32),
    pub widget: DrawingArea,
    pub backend: Sender<BKCommand>,
    pub pixbuf: Arc<Mutex<Option<Pixbuf>>>,
    /// useful to avoid the scale_simple call on every draw
    pub scaled: Arc<Mutex<Option<Pixbuf>>>,
    pub thumb: bool,
}

impl Image {
    pub fn new(backend: &Sender<BKCommand>, path: &str, size: (i32, i32), Thumb(thumb): Thumb) -> Image {
        let da = DrawingArea::new();
        let pixbuf = match gtk::IconTheme::get_default() {
            None => None,
            Some(i1) => match i1.load_icon("image-loading-symbolic", size.1, gtk::IconLookupFlags::empty()) {
                Err(_) => None,
                Ok(i2) => i2,
            }
        };

        let img = Image {
            path: path.to_string(),
            max_size: size,
            widget: da,
            pixbuf: Arc::new(Mutex::new(pixbuf)),
            scaled: Arc::new(Mutex::new(None)),
            thumb: thumb,
            backend: backend.clone(),
        };
        img.draw();
        img.load_async();

        img
    }

    pub fn draw(&self) {
        let da = &self.widget;

        let w = self.max_size.0;
        let h = self.max_size.1;

        da.set_hexpand(false);
        da.set_vexpand(false);

        if let Some(ref pb) = *self.pixbuf.lock().unwrap() {
            let w = pb.get_width();
            let h = pb.get_height();
            da.set_size_request(w, h);
        } else {
            da.set_size_request(w, h);
        }

        let pix = self.pixbuf.clone();
        let scaled = self.scaled.clone();
        da.connect_draw(move |da, g| {
            let width = w as f64;
            let height = h as f64;

            // Here we look for the first parent box and we adjust the widget width to the parent
            // less 10px to avoid resizing the window when we've a smaller window that the max_size
            //
            // This allow the user to resize to less than this image width dragging the window
            // border, but it's slow because we're resizing 10px each time.
            let mut rw = w;
            let mut parent: Option<gtk::Widget> = da.get_parent();
            loop {
                if parent.is_none() {
                    break;
                }

                let p = parent.unwrap();
                if p.is::<gtk::Box>() {
                    let parent_width = p.get_allocated_width();
                    let max = parent_width - 10;
                    if max < w {
                        rw = max;
                    }
                    break;
                }
                parent = p.get_parent();
            }

            let context = da.get_style_context().unwrap();

            gtk::render_background(&context, g, 0.0, 0.0, width, height);

            if let Some(ref pb) = *pix.lock().unwrap() {
                let mut pw = pb.get_width();
                let mut ph = pb.get_height();

                if pw > ph && pw > rw {
                    ph = rw * ph / pw;
                    pw = rw;
                } else if ph >= pw && ph > h {
                    pw = h * pw / ph;
                    ph = h;
                }
                da.set_size_request(pw, ph);

                let mut scaled_pix: Option<Pixbuf> = None;

                if let Some(ref s) = *scaled.lock().unwrap() {
                    if s.get_width() == pw && s.get_height() == ph {
                        scaled_pix = Some(s.clone());
                    }
                }

                if let None = scaled_pix {
                    scaled_pix = pb.scale_simple(pw, ph, gdk_pixbuf::InterpType::Bilinear);
                }

                if let Some(sc) = scaled_pix {
                    g.set_source_pixbuf(&sc, 0.0, 0.0);
                    g.rectangle(0.0, 0.0, pw as f64, ph as f64);
                    g.fill();
                    *scaled.lock().unwrap() = Some(sc);
                }
            }

            Inhibit(false)
        });
    }

    /// If `path` starts with mxc this func download the img async, in other case the image is loaded
    /// in the `image` widget scaled to size
    pub fn load_async(&self) {
        if self.path.starts_with("mxc:") {
            // asyn load
            let (tx, rx): (Sender<String>, Receiver<String>) = channel();
            let command = match self.thumb {
                false => BKCommand::GetMediaAsync(self.path.to_string(), tx),
                true => BKCommand::GetThumbAsync(self.path.to_string(), tx),
            };
            self.backend.send(command).unwrap();
            let pix = self.pixbuf.clone();
            let scaled = self.scaled.clone();
            let da = self.widget.clone();
            gtk::timeout_add(50, move || match rx.try_recv() {
                Err(TryRecvError::Empty) => gtk::Continue(true),
                Err(TryRecvError::Disconnected) => gtk::Continue(false),
                Ok(fname) => {
                    load_pixbuf(pix.clone(), scaled.clone(), da.clone(), &fname);
                    gtk::Continue(false)
                }
            });
        } else {
            load_pixbuf(self.pixbuf.clone(), self.scaled.clone(), self.widget.clone(), &self.path);
        }
    }
}

pub fn load_pixbuf(pix: Arc<Mutex<Option<Pixbuf>>>, scaled: Arc<Mutex<Option<Pixbuf>>>, widget: DrawingArea, fname: &str) {
    if is_gif(&fname) {
        load_animation(pix.clone(), scaled.clone(), widget, &fname);
        return;
    }

    match Pixbuf::new_from_file(fname) {
        Ok(px) => {
            *pix.lock().unwrap() = Some(px);
            *scaled.lock().unwrap() = None;
        }
        _ => {
            *pix.lock().unwrap() = None;
            *scaled.lock().unwrap() = None;
        }
    };
}

pub fn load_animation(pix: Arc<Mutex<Option<Pixbuf>>>, scaled: Arc<Mutex<Option<Pixbuf>>>, widget: DrawingArea, fname: &str) {
    let res = PixbufAnimation::new_from_file(fname);
    if res.is_err() {
        return;
    }
    let anim = res.unwrap();
    let iter = anim.get_iter(&glib::get_current_time());

    gtk::timeout_add(iter.get_delay_time() as u32, move || {
        iter.advance(&glib::get_current_time());

        if widget.is_drawable() {
            let px = iter.get_pixbuf();
            *pix.lock().unwrap() = Some(px);
            *scaled.lock().unwrap() = None;
            widget.queue_draw();
        } else {
            return gtk::Continue(false);
        }
        gtk::Continue(true)
    });
}

pub fn is_gif(fname: &str) -> bool {
    let p = &Path::new(fname);
    if !p.is_file() {
        return false;
    }
    let result = tree_magic::from_filepath(p);
    result == "image/gif"
}
