// Copyright (C) 2021 Saadi Save, Varun Jain
// Licenced under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::inits::update;
use super::responses::{ACCEPTED, BADCAND, BADKEY};
use super::types::{Accepted, BadRequest, Gpayload};
use rocket::State;

#[get("/vote/<key>/<name>")]
pub async fn vote(
    payload: State<'_, Gpayload>,
    key: String,
    name: String,
) -> Result<Accepted, BadRequest> {
    let mut p = payload.lock().await;

    if !(p.key == key) {
        return Err(BADKEY);
    }

    if p.count.contains(&name) {
        p.count.increment(&name);
        update(&p.datafile, &p.count).await;
    } else {
        return Err(BADCAND);
    }

    Ok(ACCEPTED)
}
