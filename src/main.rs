
extern crate glib;
extern crate gio;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
mod util;
mod globals;
mod widgets;
mod error;
mod types;
mod cache;
mod backend;
mod model;
mod app;
mod static_resources;

use app::App;


fn main() {
    static_resources::init().expect("GResource initialization failed.");
    App::new();
}
