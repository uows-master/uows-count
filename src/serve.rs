// Copyright (C) 2021 Saadi Save, Varun Jain
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use super::inits::*;
use super::routes::*;
use super::types::{Conf, Gcandidates, Gpayload, Payload};

pub async fn serve(conf: &Conf) -> rocket::Rocket {
    let k = init_key(&conf.keyfile).await;
    let dfile = conf.datafile.as_ref().unwrap().to_owned();
    let cnt = match conf.reset.unwrap() {
        true => reset_n_init(&conf.candidatesfile, &dfile).await,
        false => init(&dfile).await,
    };

    let payload = Gpayload::new(Payload {
        count: cnt.clone(),
        key: k.clone(),
        datafile: dfile.clone(),
    });

    let candidates = Gcandidates::new(init_candidates(&conf.candidatesfile).await);

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
        .manage(payload)
        .manage(candidates)
        .mount("/", routes![vote, get_count, get_candidates])
}
