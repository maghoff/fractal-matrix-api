extern crate regex;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;

use self::regex::Regex;
use self::gdk_pixbuf::Pixbuf;
use failure::Error;
use self::gdk::ContextExt;

pub fn markup(s: &str) -> String {
    let mut out = String::from(s);

    out = String::from(out.trim());
    out = out.replace('&', "&amp;");
    out = out.replace('<', "&lt;");
    out = out.replace('>', "&gt;");

    let re = Regex::new("(?P<url>https?://[^\\s&,)(\"]+(&\\w=[\\w._-]?)*(#[\\w._-]+)?)").unwrap();
    out = String::from(re.replace_all(&out, "<a href=\"$url\">$url</a>"));

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
