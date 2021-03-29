// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::inits::update;
use super::responses::{ACCEPTED, BADCAND, BADKEY};
use super::types::{
    data::{Candidates, Counter, DataFile, Key},
    response::{Accepted, BadRequest, Json},
};
use rocket::State;

#[get("/vote/<key>/<name>")]
pub async fn vote(
    authkey: State<'_, Key>,
    counter: State<'_, Counter>,
    datafile: State<'_, DataFile>,
    key: String,
    name: String,
) -> Result<Accepted, BadRequest> {
    if authkey.0 != key {
        return Err(BADKEY);
    }

    if counter.contains(&name) {
        counter.increment(&name);
        update(&datafile.0, &counter).await;
    } else {
        return Err(BADCAND);
    }

    Ok(ACCEPTED)
}

#[get("/candidates/<key>")]
pub async fn get_candidates(
    authkey: State<'_, Key>,
    candidates: State<'_, Candidates>,
    key: String,
) -> Result<Json, BadRequest> {
    if authkey.0 != key {
        return Err(BADKEY);
    }

    Ok(Json::from(candidates.inner()))
}

#[get("/count/<key>")]
pub async fn get_count(
    authkey: State<'_, Key>,
    counter: State<'_, Counter>,
    key: String,
) -> Result<Json, BadRequest> {
    if authkey.0 != key {
        return Err(BADKEY);
    }

    Ok(Json::from(counter.inner()))
}
