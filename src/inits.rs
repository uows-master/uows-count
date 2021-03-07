// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::types::{Candidates, Conf, Counter, InMap};
use clap::{load_yaml, App};
use rocket::tokio::fs::{read_to_string, write};

pub async fn init(datafile: &str) -> Counter {
    let s = read_to_string(datafile).await.unwrap();

    let x: InMap = serde_json::from_str(s.as_str()).unwrap();

    Counter::new(x)
}

pub async fn init_candidates(candidatesfile: &str) -> Candidates {
    let s = read_to_string(candidatesfile).await.unwrap();

    let v: Vec<String> = s.split('\n').map(std::string::ToString::to_string).collect();

    Candidates::new(v)
}

pub async fn reset_n_init(candidatesfile: &str, datafile: &str) -> Counter {
    let mut x = InMap::new();

    let s = read_to_string(candidatesfile).await.unwrap();

    for i in s.split('\n') {
        x.insert(i.to_string(), 0);
    }

    let cnt = Counter::new(x);

    write(datafile, serde_json::to_string(&cnt).unwrap())
        .await
        .unwrap();

    cnt
}

pub async fn update(datafile: &str, count: &Counter) {
    write(datafile, serde_json::to_string(count).unwrap())
        .await
        .unwrap();
}

pub async fn init_key(keyfile: &str) -> String {
    read_to_string(keyfile).await.unwrap()
}

pub async fn parse_args() -> Conf {
    let matches = App::from(load_yaml!("cli.yml")).get_matches();

    let mut conf = init_conf(matches.value_of("config").unwrap()).await;

    if matches.is_present("reset") {
        conf.reset = Some(true)
    } else {
        conf.reset = Some(false)
    }

    conf.port = match conf.port {
        Some(i) => Some(i),
        None => Some(8000),
    };

    conf.address = match conf.address {
        Some(s) => Some(s),
        None => Some("127.0.0.1".to_string()),
    };

    conf.datafile = match conf.datafile {
        Some(s) => Some(s),
        None => Some("count.json".to_string()),
    };

    conf.log_level = match conf.log_level {
        Some(i) => Some(i),
        None => Some(matches.occurrences_of("log") as u8),
    };

    conf.secure = match conf.secure {
        Some(b) => Some(b),
        None => Some(false),
    };

    conf.check();

    conf
}

async fn init_conf(confile: &str) -> Conf {
    let s = read_to_string(confile).await.unwrap();

    let x: Conf = toml::from_str(s.as_str()).unwrap();

    x
}
