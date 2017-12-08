extern crate serde_json;

use globals;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use error::Error;
use util::json_q;
use util::get_user_avatar;
use backend::types::BKResponse;
use backend::types::Backend;

use self::serde_json::Value as JsonValue;

pub fn get_username(bk: &Backend) -> Result<(), Error> {
    let id = bk.data.lock().unwrap().user_id.clone();
    let url = bk.url(&format!("profile/{}/displayname", id.clone()), vec![])?;
    let tx = bk.tx.clone();
    get!(&url,
        |r: JsonValue| {
            let name = String::from(r["displayname"].as_str().unwrap_or(&id));
            tx.send(BKResponse::Name(name)).unwrap();
        },
        |err| { tx.send(BKResponse::UserNameError(err)).unwrap() }
    );

    Ok(())
}

pub fn get_avatar(bk: &Backend) -> Result<(), Error> {
    let baseu = bk.get_base_url()?;
    let userid = bk.data.lock().unwrap().user_id.clone();

    let tx = bk.tx.clone();
    thread::spawn(move || match get_user_avatar(&baseu, &userid) {
        Ok((_, fname)) => {
            tx.send(BKResponse::Avatar(fname)).unwrap();
        }
        Err(err) => {
            tx.send(BKResponse::AvatarError(err)).unwrap();
        }
    });

    Ok(())
}

pub fn get_user_info_async(bk: &mut Backend,
                           uid: &str,
                           tx: Sender<(String, String)>)
                           -> Result<(), Error> {
    let baseu = bk.get_base_url()?;

    let u = String::from(uid);

    if let Some(info) = bk.user_info_cache.get(&u) {
        let i = info.lock().unwrap().clone();
        if !i.0.is_empty() || !i.1.is_empty() {
            tx.send(i).unwrap();
            return Ok(())
        }
    }

    let info = Arc::new(Mutex::new((String::new(), String::new())));
    let cache_key = u.clone();
    let cache_value = info.clone();

    thread::spawn(move || {
        let i0 = info.lock();
        match get_user_avatar(&baseu, &u) {
            Ok(info) => {
                tx.send(info.clone()).unwrap();
                let mut i = i0.unwrap();
                i.0 = info.0;
                i.1 = info.1;
            }
            Err(_) => {
                tx.send((String::new(), String::new())).unwrap();
            }
        };
    });

    bk.user_info_cache.insert(cache_key, cache_value);

    Ok(())
}
