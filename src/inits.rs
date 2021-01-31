#![allow(dead_code)]
use mongodb::{bson::doc, Database};
use tokio::{fs::OpenOptions, io::AsyncReadExt};
use uows_crypto::Data;

pub async fn init_candidates(db: Database, clistfile: &str) {
    let collection = db.collection("candidates");

    let mut candidates = String::new();

    OpenOptions::new()
        .read(true)
        .open(clistfile)
        .await
        .unwrap()
        .read_to_string(&mut candidates)
        .await
        .unwrap();

    for c in candidates.split('\n') {
        collection
            .insert_one(doc! {"name": c, "votes": 0}, None)
            .await
            .unwrap();
    }
}

pub async fn init_enc(keyfile: &str) -> Data {
    let mut keystr = String::new();

    OpenOptions::new()
        .read(true)
        .open(keyfile)
        .await
        .unwrap()
        .read_to_string(&mut keystr)
        .await
        .unwrap();

    let keyvec: Vec<&str> = keystr.split('\n').collect();

    Data::new(keyvec[0], keyvec[1])
}
