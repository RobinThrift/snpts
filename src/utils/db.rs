use ::rustc_serialize::json::{Json};
use std::error::Error;
use std::path::Path;
use std::fs::{File, OpenOptions};

#[derive(RustcDecodable, RustcEncodable)]
pub struct Snippet {
    name: String,
    content: String
}

pub struct Database {
    file: File,
    path: String,
    data: Json
}

impl Database {
    pub fn new(db_path: String) -> Database {
        let mut fd = OpenOptions::new().write(true).read(true).create(true).open(Path::new(&db_path)).unwrap();

        let data = match Json.from_reader(&fd) {
            Err(reason) => { panic!("Couldn't read DB file: {}", Error::description(&reason)) },
            Ok(d) => { d }
        };

        Database { file: fd, path: db_path, data: data }
    }
}
