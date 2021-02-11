#![allow(dead_code)]
use mongodb::Database;
// use std::convert::Infallible;
// use warp::{self, http::StatusCode};

use crate::db;

pub async fn vote(name: String, db: &Database) {
    db::add_vote(name.as_str(), db).await;
}
