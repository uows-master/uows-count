use actix_web::{get, middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod inits;
use inits::*;

#[get("/candidate/{key}/{name}")]
async fn handle(k: Key, count: Gcounter, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    let key = req.match_info().get("key").unwrap();

    // println!(
    //     "skey: {}\nrkey: {}\nCandidate: {}",
    //     k.lock().unwrap().as_str(),
    //     key,
    //     name
    // );

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
    let count = init("count.json");
    let k = initkey("key");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(k.clone())
            .app_data(count.clone())
            .service(handle)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
