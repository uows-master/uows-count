// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Deserialize)]
pub struct Conf {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub candidatesfile: String,
    pub datafile: Option<String>,
    pub keyfile: String,
    pub log_level: Option<u8>,
    pub secure: Option<bool>,
    pub cert: Option<String>,
    pub pkey: Option<String>,
    pub reset: Option<bool>,
}

impl Conf {
    pub fn check(&self) {
        if self.secure != Some(false) && (self.cert == None || self.pkey == None) {
            panic!("SSL cannot be used without the ceritifcate and private key")
        }

        if self.address.as_ref().unwrap().parse::<Ipv4Addr>().is_err()
            && self.address.as_ref().unwrap().parse::<Ipv6Addr>().is_err()
        {
            panic!("The IP address entered is not a valid IPv4 or IPv6 address")
        }
    }

    pub fn get_log_level(&self) -> &str {
        match self.log_level {
            Some(i) => match i {
                0 => "off",
                1 => "critical",
                2 => "normal",
                _ => "debug",
            },
            None => "normal",
        }
    }
}
