extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate cairo;

use self::gtk::prelude::*;
pub use self::gtk::DrawingArea;
use self::gdk_pixbuf::Pixbuf;
use self::gdk::ContextExt;


pub type Avatar = gtk::Box;

pub trait AvatarExt {
    fn avatar_new(size: Option<i32>) -> gtk::Box;
    fn circle_avatar(path: String, size: Option<i32>) -> gtk::Box;
    fn clean(&self);
    fn create_da(&self, size: Option<i32>) -> DrawingArea;

    fn circle(&self, path: String, size: Option<i32>);
    fn default(&self, icon: String, size: Option<i32>);
}

impl AvatarExt for gtk::Box {
    fn clean(&self) {
        for ch in self.get_children().iter() {
            self.remove(ch);
        }
    }

    fn create_da(&self, size: Option<i32>) -> DrawingArea {
        let da = DrawingArea::new();
        let s = size.unwrap_or(40);
        da.set_size_request(s, s);
        self.pack_start(&da, true, true, 0);
        self.show_all();

        da
    }

    fn avatar_new(size: Option<i32>) -> gtk::Box {
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        b.create_da(size);
        b.show_all();

        b
    }

    fn circle_avatar(path: String, size: Option<i32>) -> gtk::Box {
        let b = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        b.create_da(size);
        b.circle(path, size);
        b.show_all();

        b
    }

    fn default(&self, icon: String, size: Option<i32>) {
        self.clean();
        let da = self.create_da(size);

        da.connect_draw(move |da, g| {
            use std::f64::consts::PI;

            let width = da.get_allocated_width() as f64;
            let height = da.get_allocated_height() as f64;

            let context = da.get_style_context().unwrap();

            gtk::render_background(&context, g, 0.0, 0.0, width, height);
            g.set_antialias(cairo::Antialias::Best);

            let img = gtk::Image::new_from_icon_name(&icon[..], 5);
            let pb = img.get_pixbuf().unwrap();
            let hpos: f64 = (width - (pb.get_height()) as f64) / 2.0;

            g.set_source_pixbuf(&pb, 0.0, hpos);
            g.rectangle(0.0, 0.0, width, height);
            g.fill();

            g.arc(width / 2.0, height / 2.0, width.min(height) / 2.5, 0.0, 2.0 * PI);
            g.clip();

            Inhibit(false)
        });
    }

    fn circle(&self, path: String, size: Option<i32>) {
        self.clean();
        let da = self.create_da(size);

        let p = path.clone();
        da.connect_draw(move |da, g| {
            use std::f64::consts::PI;

            let width = da.get_allocated_width() as f64;
            let height = da.get_allocated_height() as f64;

            let context = da.get_style_context().unwrap();

            gtk::render_background(&context, g, 0.0, 0.0, width, height);
            g.set_antialias(cairo::Antialias::Best);

            let pb = Pixbuf::new_from_file_at_scale(&p, width as i32, -1, true).unwrap();
            let hpos: f64 = (width - (pb.get_height()) as f64) / 2.0;

            g.arc(width / 2.0, height / 2.0, width.min(height) / 2.0, 0.0, 2.0 * PI);
            g.clip();

            g.set_source_pixbuf(&pb, 0.0, hpos);
            g.rectangle(0.0, 0.0, width, height);
            g.fill();

            Inhibit(false)
        });
    }
}
