// Copyright (C) 2021 Saadi Save, Varun Jain
// Licenced under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use rocket::http::ContentType;
use rocket::tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type Inmap = BTreeMap<String, u32>;
pub type Gpayload = Mutex<Payload>;

#[derive(Deserialize)]
pub struct Conf {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub candidatesfile: String,
    pub datafile: Option<String>,
    pub keyfile: String,
    pub log_level: Option<u8>,
    pub secure: Option<bool>,
    pub cert: Option<String>,
    pub pkey: Option<String>,
    pub reset: Option<bool>,
}

impl Conf {
    pub fn get_log_level(&self) -> &str {
        match self.log_level {
            Some(i) => match i {
                0 => "off",
                1 => "critical",
                2 => "normal",
                3 => "debug",
                _ => "debug",
            },
            None => "normal",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Counter(Inmap);

impl Counter {
    pub fn new(i: Inmap) -> Counter {
        Counter(i)
    }
    pub fn increment(&mut self, name: &str) {
        *self.0.entry(name.to_string()).or_insert(0) += 1;
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }
}

#[derive(Responder)]
#[response(status = 202, content_type = "text/html")]
pub struct Accepted {
    pub inner: &'static str,
    pub header: ContentType,
}

#[derive(Responder)]
#[response(status = 400, content_type = "text/html")]
pub struct BadRequest {
    pub inner: &'static str,
    pub header: ContentType,
}

#[derive(Clone)]
pub struct Payload {
    pub count: Counter,
    pub key: String,
    pub datafile: String,
}
