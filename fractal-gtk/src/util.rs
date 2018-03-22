extern crate regex;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;

use self::regex::Regex;
use self::gdk_pixbuf::Pixbuf;
use self::gdk_pixbuf::PixbufExt;
use failure::Error;
use self::gdk::ContextExt;

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

/// Converts the input `&str` to pango format, replacing special characters
/// `&, < and >` and parses URLS to show as a link
///
/// # Examples
///
/// ```
/// let m = markup("this is parsed");
/// assert_eq!(&m, "this is parsed");
///
/// let m = markup("this is <span>parsed</span>");
/// assert_eq!(&m, "this is &lt;parsed&gt;");
///
/// let m = markup();
/// assert_eq!(&m, "with links: <a href=\"http://gnome.org\">http://gnome.org</a> ");
/// ```
pub fn markup(s: &str) -> String {
    let mut out = String::from(s);

    out = String::from(out.trim());
    out = out.replace('&', "&amp;");
    out = out.replace('<', "&lt;");
    out = out.replace('>', "&gt;");

    let amp = "(&amp;)";
    let domain = "[^\\s,)(\"]+";
    let param = format!("({amp}?\\w+(=[\\w._-]+)?)", amp=amp);
    let params = format!("(\\?{param}*)*", param=param);
    let hash = "(#[\\w._-]+)?";

    let regex_str = format!("(https?://{domain}{params}{hash})", domain=domain, params=params, hash=hash);

    let re = Regex::new(&regex_str).unwrap();
    out = String::from(re.replace_all(&out, "<a href=\"$0\">$0</a>"));

    out
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_markup() {
        let m = markup("this is parsed");
        assert_eq!(&m, "this is parsed");

        let m = markup("this is <span>parsed</span>");
        assert_eq!(&m, "this is &lt;span&gt;parsed&lt;/span&gt;");

        let m = markup("this is &ssdf;");
        assert_eq!(&m, "this is &amp;ssdf;");

        let url = "http://url.com/test?param1&param2=test&param3#hashing";
        let m = markup(&format!("this is &ssdf; {}", url));
        assert_eq!(&m, &format!("this is &amp;ssdf; <a href=\"{0}\">{0}</a>", url.replace('&', "&amp;")));

        for l in &[
           ("with links: http://gnome.org :D", "http://gnome.org"),
           ("with links: http://url.com/test.html&stuff :D", "http://url.com/test.html&stuff"),
           ] {
            let m = markup(l.0);
            assert_eq!(&m, &format!("with links: <a href=\"{0}\">{0}</a> :D", l.1.replace('&', "&amp;")));
        }
    }
}
