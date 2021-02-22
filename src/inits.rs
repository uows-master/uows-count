use std::{
    fs::{read_to_string, write},
    sync::Mutex,
};

mod types;
pub use types::*;

pub fn init(path: &str) -> Gcounter {
    let s = read_to_string(path).unwrap();
    let x: InnerMap = serde_json::from_str(s.as_str()).unwrap();

    Gcounter::new(Mutex::new(Counter::new(x)))
}

pub fn update(path: &str, count: &Counter) {
    let s = serde_json::to_string(count).unwrap();
    write(path, s.as_str()).unwrap();
}

pub fn initkey(path: &str) -> Key {
    Key::new(Mutex::new(read_to_string(path).unwrap()))
}
