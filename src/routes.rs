// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::inits::update;
use super::responses::{ACCEPTED, BADCAND, BADKEY};
use super::types::{Accepted, BadRequest, GCandidates, GCounter, JsonResponse, Payload};
use rocket::State;

#[get("/vote/<key>/<name>")]
pub async fn vote(
    counter: State<'_, GCounter>,
    payload: State<'_, Payload>,
    key: String,
    name: String,
) -> Result<Accepted, BadRequest> {
    if payload.key != key {
        return Err(BADKEY);
    }

    if counter.count.contains(&name) {
        counter.count.increment(&name);
        update(&payload.datafile, &counter.count).await;
    } else {
        return Err(BADCAND);
    }

    Ok(ACCEPTED)
}

#[get("/count/<key>")]
pub async fn get_count(
    counter: State<'_, GCounter>,
    payload: State<'_, Payload>,
    key: String,
) -> Result<JsonResponse, BadRequest> {
    if payload.key != key {
        return Err(BADKEY);
    }

    Ok(JsonResponse::from(&counter.count))
}

#[get("/candidates/<key>")]
pub async fn get_candidates(
    gcandidates: State<'_, GCandidates>,
    key: String,
) -> Result<JsonResponse, BadRequest> {
    if gcandidates.key != key {
        return Err(BADKEY);
    }

    Ok(JsonResponse::from(&gcandidates.candidates))
}
