// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::types::{
    config::Conf,
    data::{Candidates, Counter, InMap},
};
use clap::{load_yaml, App};
use rocket::tokio::fs::{read_to_string, write};
use std::sync::atomic::AtomicU32;

pub async fn init(datafile: &str) -> Counter {
    let s = read_to_string(datafile).await.unwrap();

    let x: InMap = serde_json::from_str(s.as_str()).unwrap();

    Counter::new(x)
}

pub async fn init_candidates(candidatesfile: &str) -> Candidates {
    let s = read_to_string(candidatesfile).await.unwrap();

    let v: Vec<String> = s
        .split('\n')
        .map(std::string::ToString::to_string)
        .collect();

    Candidates::new(v)
}

pub async fn init_conf(confile: &str) -> Conf {
    let s = read_to_string(confile).await.unwrap();

    let x: Conf = toml::from_str(s.as_str()).unwrap();

    x
}

pub async fn init_key(keyfile: &str) -> String {
    read_to_string(keyfile).await.unwrap()
}

pub async fn reset_n_init(candidatesfile: &str, datafile: &str) -> Counter {
    let mut x = InMap::new();

    let s = read_to_string(candidatesfile).await.unwrap();

    for i in s.split('\n') {
        x.insert(i.to_string(), AtomicU32::from(0));
    }

    let cnt = Counter::new(x);

    update(datafile, &cnt).await;

    cnt
}

pub async fn update(datafile: &str, count: &Counter) {
    write(datafile, serde_json::to_string(count).unwrap())
        .await
        .unwrap();
}

pub async fn parse_args() -> Conf {
    let matches = App::from(load_yaml!("cli.yml")).get_matches();

    let mut conf = init_conf(matches.value_of("config").unwrap()).await;

    conf.reset = Some(conf.reset.unwrap_or(matches.is_present("reset")));

    conf.port = Some(conf.port.unwrap_or(8000));

    conf.address = Some(conf.address.unwrap_or("127.0.0.1".to_string()));

    conf.datafile = Some(conf.datafile.unwrap_or("count.json".to_string()));

    conf.log_level = Some(
        conf.log_level
            .unwrap_or(matches.occurrences_of("log") as u8),
    );

    conf.secure = Some(conf.secure.unwrap_or(false));

    conf.check();

    conf
}
