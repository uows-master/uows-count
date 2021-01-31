#![allow(dead_code)]
use mongodb::{bson::doc, Client, Database};
use tokio::{fs::OpenOptions, io::AsyncReadExt};

pub async fn init(cfilepath: &str) -> Database {
    let mut file = OpenOptions::new().read(true).open(cfilepath).await.unwrap();

    let mut filecontent = String::new();
    file.read_to_string(&mut filecontent).await.unwrap();

    let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
        .await
        .unwrap();

    let db = client.database("UOWS");

    let coll = db.collection("testone");

    coll.delete_many(doc! {}, None).await.unwrap();

    for i in filecontent.split('\n') {
        coll.insert_one(doc! { "name": i, "votes": 0 }, None)
            .await
            .unwrap();
    }

    db
}
