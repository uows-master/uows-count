mod db;
mod handlers;
mod inits;
mod routes;

// use mongodb::bson::doc;
use tokio::time::Instant;
use warp::{self, http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    // The current server can only receive a name at
    // localhost:5000/candidate/<name> and print it to the console

    let now = Instant::now();

    let dbase = db::init(([127, 0, 0, 1], 27017), "UOWS").await;
    inits::init_candidates(&dbase, "clist.txt").await;

    let vote = warp::path("candidate")
        .and(warp::path::param())
        .map(|name: String| {
            println!("{}", &name);
            StatusCode::ACCEPTED
        });

    warp::serve(vote).run(([127, 0, 0, 1], 5000)).await;

    /* let reciept = dbase
    .collection("candidates")
    .find(doc! {}, None)
    .await
    .unwrap(); */

    let then = now.elapsed();

    // println!("{:#?}", reciept);
    println!("Time: {:?}", then);
}
