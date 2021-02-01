mod db;
mod inits;

use mongodb::bson::doc;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let now = Instant::now();

    // Example init
    let dbase = db::init(([127, 0, 0, 1], 27017), "UOWS").await;
    inits::init_candidates(&dbase, "clist.txt").await;
    
    let reciept = dbase
        .collection("candidates")
        .find(doc! {}, None)
        .await
        .unwrap();

    let then = now.elapsed();
    
    println!("{:#?}", reciept);
    println!("Time: {:?}", then);
}
