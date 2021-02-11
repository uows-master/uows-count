#![allow(dead_code)]
use mongodb::{
    bson::{doc, Bson},
    Client, Database,
};

pub async fn init(bind: ([u8; 4], u16), dbname: &str) -> Database {
    let client = Client::with_uri_str(
        format!(
            "mongodb://{}.{}.{}.{}:{}",
            bind.0[0], bind.0[1], bind.0[2], bind.0[3], bind.1
        )
        .as_str(),
    )
    .await
    .unwrap();

    let db = client.database(dbname);

    db
}

pub async fn add_vote(name: &str, db: &Database) {
    let votes = db.collection("candidates");
    let mut nvotes = 0;
    let candidate = votes.find_one(doc! { "name" : name}, None).await.unwrap();

    match candidate {
        Some(doc) => {
            if let Some(votecnt) = doc.get("votes").and_then(Bson::as_i32) {
                nvotes = votecnt
            }
        }
        _ => {}
    };

    nvotes += 1;

    votes
        .update_one(doc! { "name" : name }, doc! { "votes" : nvotes }, None)
        .await
        .unwrap();
}
