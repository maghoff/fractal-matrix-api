extern crate cairo;
extern crate url;
extern crate regex;
extern crate reqwest;
extern crate glib;
extern crate serde_json;

use std::io;

#[derive(Debug)]
pub enum Error {
    BackendError,
    CacheError,
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

derror!(url::ParseError, Error::BackendError);
derror!(io::Error, Error::BackendError);
derror!(regex::Error, Error::BackendError);
derror!(cairo::Status, Error::BackendError);
derror!(cairo::IoError, Error::BackendError);
derror!(cairo::BorrowError, Error::BackendError);
derror!(glib::Error, Error::BackendError);

derror!(serde_json::Error, Error::CacheError);
