// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use crate::inits;
use crate::types;

#[tokio::test]
async fn init_dfile() {
    let x = inits::init("count.json").await;

    assert_eq!(x.contains("CDU"), true);
    assert_eq!(x.contains("SPD"), true);
    assert_eq!(x.contains("AfD"), true);
    assert_eq!(x.contains("FDP"), true);
    assert_eq!(x.contains("B90G"), true);
}

#[tokio::test]
async fn reset_and_update() {
    let x = inits::init("count.json").await;

    let a = inits::init("count.json").await;

    x.increment("CDU");

    inits::update("count.json", &x).await;

    let y = inits::init("count.json").await;

    assert_eq!(
        serde_json::to_string(&x).unwrap(),
        serde_json::to_string(&y).unwrap()
    );

    let b = inits::reset_n_init("candidates", "count.json").await;

    assert_eq!(
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&b).unwrap()
    )
}

#[tokio::test]
async fn conf() {
    let x = inits::init_conf("Config.toml").await;

    assert_eq!(x.address.unwrap(), "::");
    assert_eq!(x.port.unwrap(), 8080);
    assert_eq!(x.candidatesfile, "candidates");
    assert_eq!(x.keyfile, "key");
}

#[tokio::test]
async fn key() {
    assert_eq!(inits::init_key("key").await, "4b80");
}

#[tokio::test]
async fn candidates() {
    let x = vec![
        "CDU".to_string(),
        "AfD".to_string(),
        "SPD".to_string(),
        "FDP".to_string(),
        "B90G".to_string(),
        "DL".to_string(),
    ];

    assert_eq!(
        inits::init_candidates("candidates").await,
        types::data::Candidates::new(x)
    );
}
