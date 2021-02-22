use actix_web::{get, middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::App as Capp;
use clap::load_yaml;

mod inits;
use inits::*;

#[get("/candidate/{key}/{name}")]
async fn handle(k: Key, count: Gcounter, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let key = req.match_info().get("key").unwrap();

    if k.lock().unwrap().as_str() != key {
        HttpResponse::BadRequest().body("<h1>Error 400 Bad Request</h1><p>Invalid key</p>")
    } else {
        let mut cnt = count.lock().unwrap();
        if cnt.contains(name) {
            cnt.increment(name);
            update("count.json", &cnt);
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
    let matches = Capp::from_yaml(conf).get_matches();
    
    let bind = matches.value_of("bind").unwrap();
    let keyf = matches.value_of("keyfile").unwrap();
    let clist = matches.value_of("candidatelist").unwrap();
    let log = matches.is_present("log");
    
    if matches.is_present("sslcert") {
        panic!("SSL is not currently implemented");
    };

    let count = init(clist);
    let k = initkey(keyf);

    if log {
        println!("Logging is still not functioning properly");
        println!("Server has started");
        std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
        std::env::set_var("RUST_BACKTRACE", "1");
        HttpServer::new(move || {
            App::new()
                .app_data(k.clone())
                .app_data(count.clone())
                .wrap(Logger::default())
                .service(handle)
        }).bind(bind).unwrap().run().await
    } else {
        println!("Server has started");
        HttpServer::new(move || {
            App::new()
                .app_data(k.clone())
                .app_data(count.clone())
                .service(handle)
        }).bind(bind).unwrap().run().await
    }
}
