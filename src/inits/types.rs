use actix_web::web;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Mutex};

pub type Gcounter = web::Data<Mutex<Counter>>;
pub type Key = web::Data<Mutex<String>>;
pub type InnerMap = BTreeMap<String, u32>;

#[derive(Serialize, Deserialize)]
pub struct Counter(InnerMap);

impl Counter {
    pub fn new(i: InnerMap) -> Counter {
        Counter(i)
    }
    pub fn increment(&mut self, name: &str) {
        *self.0.entry(name.to_string()).or_insert(0) += 1;
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }
}
