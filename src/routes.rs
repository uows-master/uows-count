// Copyright (C) 2021 Saadi Save, Varun Jain
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::inits::update;
use super::responses::{ACCEPTED, BADCAND, BADKEY};
use super::types::{Accepted, BadRequest, Candidates, Counter, Gcandidates, Gpayload};
use rocket::State;
use rocket_contrib::json::Json;

#[get("/vote/<key>/<name>")]
pub async fn vote(
    payload: State<'_, Gpayload>,
    key: String,
    name: String,
) -> Result<Accepted, BadRequest> {
    let mut pld = payload.lock().await;

    if !(pld.key == key) {
        return Err(BADKEY);
    }

    if pld.count.contains(&name) {
        pld.count.increment(&name);
        update(&pld.datafile, &pld.count).await;
    } else {
        return Err(BADCAND);
    }

    Ok(ACCEPTED)
}

#[get("/<key>/count")]
pub async fn get_count(
    payload: State<'_, Gpayload>,
    key: String,
) -> Result<Json<Counter>, BadRequest> {
    let pld = payload.lock().await;

    if pld.key != key {
        return Err(BADKEY);
    }

    Ok(Json(pld.count.clone()))
}

#[get("/<key>/candidates")]
pub async fn get_candidates(
    payload: State<'_, Gpayload>,
    candidates: State<'_, Gcandidates>,
    key: String,
) -> Result<Json<Candidates>, BadRequest> {
    if payload.lock().await.key != key {
        return Err(BADKEY);
    }

    Ok(Json(candidates.lock().await.clone()))
}
