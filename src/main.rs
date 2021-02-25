// Copyright (C) 2021 Saadi Save, Varun Jain
// Licenced under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

use actix_web::{get, middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::load_yaml;
use clap::App as Capp;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod inits;
use inits::*;

#[get("/candidate/{key}/{name}")]
async fn handle(k: Key, count: Gcounter, cpath: Internalpath, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let key = req.match_info().get("key").unwrap();

    if k.lock().unwrap().as_str() != key {
        HttpResponse::BadRequest().body("<h1>Error 400 Bad Request</h1><p>Invalid key</p>")
    } else {
        let mut cnt = count.lock().unwrap();
        if cnt.contains(name) {
            cnt.increment(name);
            update(
                String::from_utf8(cpath.lock().unwrap().to_vec())
                    .unwrap()
                    .as_str(),
                &cnt,
            );
            HttpResponse::Accepted().body("<h1>Success 202 Accepted</h1><p>Your vote is being processed. It is guaranteed to be counted</p>")
        } else {
            HttpResponse::BadRequest()
                .body("<h1>Error 400 Bad Request</h1><p>Invalid Candidate name</p>")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = load_yaml!("cli.yml");
    let matches = Capp::from(conf).get_matches();

    let bind = matches.value_of("bind").unwrap();
    let keyf = matches.value_of("keyfile").unwrap();
    let clist = matches.value_of("candidatelist").unwrap();
    let cpres = matches.is_present("countfile");
    let log = matches.is_present("log");
    let sec = matches.is_present("secure");
    let cfile: &str;

    if cpres {
        cfile = matches.value_of("countfile").unwrap()
    } else {
        cfile = "count.json"
    }

    let count = match matches.is_present("init") {
        true => initwreset(clist, cfile),
        false => match cpres {
            true => init(cfile),
            false => panic!("-i flag MUST be present if the countfile is not passed."),
        },
    };

    let k = initkey(keyf);

    let cpath = mk_cnt_path(if cpres { cfile } else { "count.json" });

    if log {
        stderrlog::new()
            .module("actix_web::middleware")
            .verbosity(2)
            .init()
            .expect("logger failed");

        if sec {
            if !matches.is_present("privkey") || !matches.is_present("ssl cert") {
                panic!("Certficate and private key MUST be provided");
            }

            let pkey = matches.value_of("privkey").unwrap();
            let cert = matches.value_of("sslcert").unwrap();

            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(pkey, SslFiletype::PEM)
                .unwrap();
            builder.set_certificate_chain_file(cert).unwrap();

            println!("Server has started");

            HttpServer::new(move || {
                App::new()
                    .app_data(k.clone())
                    .app_data(count.clone())
                    .app_data(cpath.clone())
                    .wrap(Logger::new("%a %r %s %b bytes"))
                    .service(handle)
            })
            .bind_openssl(bind, builder)
            .unwrap()
            .run()
            .await
        } else {
            println!("Server has started");

            HttpServer::new(move || {
                App::new()
                    .app_data(k.clone())
                    .app_data(count.clone())
                    .app_data(cpath.clone())
                    .wrap(Logger::new("%a %r %s %b bytes"))
                    .service(handle)
            })
            .bind(bind)
            .unwrap()
            .run()
            .await
        }
    } else {
        if sec {
            if !matches.is_present("privkey") || !matches.is_present("ssl cert") {
                panic!("Certficate and private key MUST be provided");
            }

            let pkey = matches.value_of("privkey").unwrap();
            let cert = matches.value_of("sslcert").unwrap();

            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(pkey, SslFiletype::PEM)
                .unwrap();
            builder.set_certificate_chain_file(cert).unwrap();

            println!("Server has started");

            HttpServer::new(move || {
                App::new()
                    .app_data(k.clone())
                    .app_data(count.clone())
                    .app_data(cpath.clone())
                    .service(handle)
            })
            .bind_openssl(bind, builder)
            .unwrap()
            .run()
            .await
        } else {
            println!("Server has started");

            HttpServer::new(move || {
                App::new()
                    .app_data(k.clone())
                    .app_data(count.clone())
                    .app_data(cpath.clone())
                    .service(handle)
            })
            .bind(bind)
            .unwrap()
            .run()
            .await
        }
    }
}
