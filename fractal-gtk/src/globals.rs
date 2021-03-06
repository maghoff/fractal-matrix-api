pub static INITIAL_MESSAGES: usize = 40;
pub static CACHE_SIZE: usize = 40;
pub static MSG_ICON_SIZE: i32 = 40;
pub static USERLIST_ICON_SIZE: i32 = 30;
pub static MINUTES_TO_SPLIT_MSGS: i64 = 30;
pub static APP_ID: &'static str = "org.gnome.Fractal";
pub static DEFAULT_HOMESERVER: &'static str = "https://matrix.org";
pub static DEFAULT_IDENTITYSERVER: &'static str = "https://vector.im";


include!(concat!(env!("OUT_DIR"), "/build_globals.rs"));
