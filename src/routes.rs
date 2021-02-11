#![allow(dead_code)]

use std::convert::Infallible;
use mongodb::Database;
use warp::Filter;

fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = Infallible> {
    warp::any().map(move || db.clone())
}

pub async fn vote() {}