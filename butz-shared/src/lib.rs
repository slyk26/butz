pub mod models;

pub mod utils {
    use std::io::{Error, ErrorKind};

    pub fn split_key(key: & str) -> (&str,&str) {
         match key.find(':') {
            Some(i) => (&key[..i], &key[(i + 1)..]),
            None => ("", ""),
        }
    }

    pub fn new_error(msg: &str) ->  Error {
        Error::new(ErrorKind::Other, msg)
    }
}