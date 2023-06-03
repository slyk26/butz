pub mod models;

pub mod utils {
    pub fn split_key(key: & str) -> (&str,&str) {
         match key.find(':') {
            Some(i) => (&key[..i], &key[(i + 1)..]),
            None => ("", ""),
        }
    }
}