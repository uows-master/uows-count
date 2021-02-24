// Copyright (C) 2021 Saadi Save, Varun Jain
// Licenced under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use std::{
    fs::{read_to_string, write},
    sync::Mutex,
};

mod types;
pub use types::*;

pub fn init(cfile: &str) -> Gcounter {
    let s = read_to_string(cfile).unwrap();
    let x: InnerMap = serde_json::from_str(s.as_str()).unwrap();

    Gcounter::new(Mutex::new(Counter::new(x)))
}

pub fn initwreset(clist: &str, cfile: &str) -> Gcounter {
    let mut x = InnerMap::new();
    let s = read_to_string(clist).unwrap();
    let svec: Vec<&str> = s.split('\n').collect();

    for i in svec {
        x.insert(i.to_string(), 0);
    }

    let cnt = Counter::new(x);

    write(cfile, serde_json::to_string(&cnt).unwrap()).unwrap();

    Gcounter::new(Mutex::new(cnt))
}

pub fn update(path: &str, count: &Counter) {
    let s = serde_json::to_string(count).unwrap();
    write(path, s.as_str()).unwrap();
}

pub fn initkey(path: &str) -> Key {
    Key::new(Mutex::new(read_to_string(path).unwrap()))
}

pub fn mk_cnt_path(cfile: &str) -> Internalpath {
    Internalpath::new(Mutex::new(cfile.to_string().as_bytes().to_vec()))
}
