use std::thread;
use std::sync::mpsc::Sender;
use error::Error;
use backend::types::BKResponse;
use backend::types::Backend;

use util::dw_media;

pub fn get_thumb_async(bk: &Backend, media: String, tx: Sender<String>) -> Result<(), Error> {
    let baseu = bk.get_base_url()?;

    thread::spawn(move || {
        match thumb!(&baseu, &media) {
            Ok(fname) => {
                tx.send(fname).unwrap();
            }
            Err(_) => {
                tx.send(String::from("")).unwrap();
            }
        };
    });

    Ok(())
}

pub fn get_media(bk: &Backend, media: String) -> Result<(), Error> {
    let baseu = bk.get_base_url()?;

    let tx = bk.tx.clone();
    thread::spawn(move || {
        match media!(&baseu, &media) {
            Ok(fname) => {
                tx.send(BKResponse::Media(fname)).unwrap();
            }
            Err(err) => {
                tx.send(BKResponse::MediaError(err)).unwrap();
            }
        };
    });

    Ok(())
}

