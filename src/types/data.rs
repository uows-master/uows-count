// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::atomic::AtomicU32};

pub type InMap = BTreeMap<String, AtomicU32>;

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

pub struct GCounter {
    pub count: Counter,
}

pub struct Payload {
    pub key: String,
    pub datafile: String,
}

#[derive(Serialize)]
pub struct Candidates(Vec<String>);

impl Candidates {
    pub fn new(candidates: Vec<String>) -> Candidates {
        Candidates(candidates)
    }
}

pub struct GCandidates {
    pub key: String,
    pub candidates: Candidates,
}
