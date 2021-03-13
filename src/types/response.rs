// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::data::{Candidates, Counter};
use rocket::http::ContentType;

#[derive(Responder)]
#[response(status = 202, content_type = "text/html")]
pub struct Accepted {
    pub inner: &'static str,
    pub header: ContentType,
}

#[derive(Responder)]
#[response(status = 400, content_type = "text/html")]
pub struct BadRequest {
    pub inner: &'static str,
    pub header: ContentType,
}

#[derive(Responder)]
#[response(status = 200, content_type = "application/json")]
pub struct Json {
    pub inner: String,
    pub header: ContentType,
}

impl From<&Counter> for Json {
    fn from(counter: &Counter) -> Self {
        Json {
            inner: serde_json::to_string(counter).unwrap(),
            header: ContentType::JSON,
        }
    }
}

impl From<&Candidates> for Json {
    fn from(candidates: &Candidates) -> Self {
        Json {
            inner: serde_json::to_string(candidates).unwrap(),
            header: ContentType::JSON,
        }
    }
}
