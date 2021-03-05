// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

#[macro_use]
extern crate rocket;

mod inits;
mod responses;
mod routes;
mod serve;
mod types;

#[launch]
async fn rocket() -> rocket::Rocket {
    serve::serve(&inits::parse_args().await).await
}
