// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::types::{Accepted, BadRequest};
use rocket::http::ContentType;

pub const ACCEPTED: Accepted = Accepted {
    inner: "<h1>Accepted</h1><p>Your vote has been accepted. It is guaranteed to be counted.</p>",
    header: ContentType::HTML,
};

pub const BADKEY: BadRequest = BadRequest {
    inner: "<h1>Invalid Key</h1><p>Your request doesn't use the correct key.</p>",
    header: ContentType::HTML,
};

pub const BADCAND: BadRequest = BadRequest {
    inner: "<h1>Invalid Candidate Name</h1><p>Your request doesn't use a valid candidate name.</p>",
    header: ContentType::HTML,
};
