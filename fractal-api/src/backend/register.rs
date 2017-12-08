extern crate url;
extern crate serde_json;

use self::serde_json::Value as JsonValue;

use std::thread;
use self::url::Url;

use util::json_q;
use globals;
use error::Error;

use backend::types::BKResponse;
use backend::types::Backend;


pub fn guest(bk: &Backend, server: String) -> Result<(), Error> {
    let s = server.clone();
    let url = Url::parse(&s).unwrap().join("/_matrix/client/r0/register?kind=guest")?;
    bk.data.lock().unwrap().server_url = s;

    let data = bk.data.clone();
    let tx = bk.tx.clone();
    let attrs = json!({});
    post!(&url, &attrs,
          |r: JsonValue| {
        let uid = String::from(r["user_id"].as_str().unwrap_or(""));
        let tk = String::from(r["access_token"].as_str().unwrap_or(""));
        data.lock().unwrap().user_id = uid.clone();
        data.lock().unwrap().access_token = tk.clone();
        data.lock().unwrap().since = String::from("");
        tx.send(BKResponse::Token(uid, tk)).unwrap();
        tx.send(BKResponse::Rooms(vec![], None)).unwrap();
    },
          |err| tx.send(BKResponse::GuestLoginError(err)).unwrap());

    Ok(())
}

pub fn login(bk: &Backend, user: String, password: String, server: String) -> Result<(), Error> {
    let s = server.clone();
    bk.data.lock().unwrap().server_url = s;
    let url = bk.url("login", vec![])?;

    let attrs = json!({
        "type": "m.login.password",
        "user": user,
        "password": password
    });

    let data = bk.data.clone();

    let tx = bk.tx.clone();
    post!(&url, &attrs,
        |r: JsonValue| {
            let uid = String::from(r["user_id"].as_str().unwrap_or(""));
            let tk = String::from(r["access_token"].as_str().unwrap_or(""));

            if uid.is_empty() || tk.is_empty() {
                tx.send(BKResponse::LoginError(Error::BackendError)).unwrap();
            } else {
                data.lock().unwrap().user_id = uid.clone();
                data.lock().unwrap().access_token = tk.clone();
                data.lock().unwrap().since = String::new();
                tx.send(BKResponse::Token(uid, tk)).unwrap();
            }
        },
        |err| { tx.send(BKResponse::LoginError(err)).unwrap() }
    );

    Ok(())
}

pub fn logout(bk: &Backend) -> Result<(), Error> {
    let url = bk.url("logout", vec![])?;
    let attrs = json!({});

    let data = bk.data.clone();
    let tx = bk.tx.clone();
    post!(&url, &attrs,
        |_| {
            data.lock().unwrap().user_id = String::new();
            data.lock().unwrap().access_token = String::new();
            data.lock().unwrap().since = String::new();
            tx.send(BKResponse::Logout).unwrap();
        },
        |err| { tx.send(BKResponse::LogoutError(err)).unwrap() }
    );
    Ok(())
}

pub fn register(bk: &Backend, user: String, password: String, server: String) -> Result<(), Error> {
    let s = server.clone();
    bk.data.lock().unwrap().server_url = s;
    let url = bk.url("register", vec![("kind", strn!("user"))])?;

    let attrs = json!({
        "auth": {"type": "m.login.password"},
        "username": user,
        "bind_email": false,
        "password": password
    });

    let data = bk.data.clone();
    let tx = bk.tx.clone();
    post!(&url, &attrs,
        |r: JsonValue| {
            println!("RESPONSE: {:#?}", r);
            let uid = String::from(r["user_id"].as_str().unwrap_or(""));
            let tk = String::from(r["access_token"].as_str().unwrap_or(""));

            data.lock().unwrap().user_id = uid.clone();
            data.lock().unwrap().access_token = tk.clone();
            data.lock().unwrap().since = String::from("");
            tx.send(BKResponse::Token(uid, tk)).unwrap();
        },
        |err| { tx.send(BKResponse::LoginError(err)).unwrap() }
    );

    Ok(())
}
