#![allow(dead_code)]
use mongodb::{bson::doc, Database};
use tokio::fs::read_to_string;
use uows_crypto::Data;

pub async fn init_candidates(db: &Database, clistfile: &str) {
    let collection = db.collection("candidates");

    let candidates = read_to_string(clistfile).await.unwrap();

    for c in candidates.split('\n') {
        collection
            .insert_one(doc! {"name": c, "votes": 0}, None)
            .await
            .unwrap();
    }
}

pub async fn init_enc(keyfile: &str) -> Data {
    let keystr = read_to_string(keyfile).await.unwrap();

    let kvec: Vec<&str> = keystr.split('\n').collect();

    Data::new(kvec[0], kvec[1])
}
