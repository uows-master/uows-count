// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::inits::{init, init_candidates, init_key, reset_n_init};
use super::routes::{get_candidates, get_count, vote};
use super::types::{
    config::Conf,
    data::{DataFile, Key},
};

pub async fn serve(conf: &Conf) -> rocket::Rocket {
    let authkey = Key(init_key(&conf.keyfile).await);
    let datafile = DataFile(conf.datafile.as_ref().unwrap().to_owned());
    let candidates = init_candidates(&conf.candidatesfile).await;

    let counter = if conf.reset.unwrap() {
        reset_n_init(&conf.candidatesfile, &datafile.0).await
    } else {
        init(&datafile.0).await
    };

    let mut figment = rocket::Config::figment()
        .merge(("address", conf.address.as_ref().unwrap()))
        .merge(("port", conf.port.unwrap()))
        .merge(("log_level", conf.get_log_level()));

    if conf.secure.unwrap() {
        figment = figment
            .merge(("tls.key", conf.pkey.as_ref().unwrap()))
            .merge(("tls.certs", conf.cert.as_ref().unwrap()));
    }

    rocket::custom(figment)
        .manage(authkey)
        .manage(candidates)
        .manage(datafile)
        .manage(counter)
        .mount("/", routes![vote, get_count, get_candidates])
}
