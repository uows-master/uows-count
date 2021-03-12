// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use rocket::http::ContentType;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::atomic::AtomicU32};

pub type InMap = BTreeMap<String, AtomicU32>;

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
    pub fn check(&self) {
        if self.secure != Some(false) && (self.cert == None || self.pkey == None) {
            panic!("SSL cannot be used without the ceritifcate and private key")
        }
    }

    pub fn get_log_level(&self) -> &str {
        match self.log_level {
            Some(i) => match i {
                0 => "off",
                1 => "critical",
                2 => "normal",
                _ => "debug",
            },
            None => "normal",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Counter(InMap);

impl Counter {
    pub fn new(i: InMap) -> Counter {
        Counter(i)
    }
    pub fn increment(&self, name: &str) {
        self.0
            .get(name)
            .unwrap()
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
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

#[derive(Responder)]
#[response(status = 200, content_type = "application/json")]
pub struct JsonResponse {
    pub inner: String,
    pub header: ContentType,
}

impl From<&Counter> for JsonResponse {
    fn from(counter: &Counter) -> Self {
        JsonResponse {
            inner: serde_json::to_string(counter).unwrap(),
            header: ContentType::JSON,
        }
    }
}

impl From<&Candidates> for JsonResponse {
    fn from(candidates: &Candidates) -> Self {
        JsonResponse {
            inner: serde_json::to_string(candidates).unwrap(),
            header: ContentType::JSON,
        }
    }
}

pub struct GCounter {
    pub count: Counter,
}

pub struct Payload {
    pub key: String,
    pub datafile: String,
}

#[derive(Serialize, Clone)]
pub struct Candidates(Vec<String>);

impl Candidates {
    pub fn new(candidates: Vec<String>) -> Candidates {
        Candidates(candidates)
    }
}

#[derive(Clone)]
pub struct GCandidates {
    pub key: String,
    pub candidates: Candidates,
}
