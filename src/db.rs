#![allow(dead_code)]
use mongodb::{Client, Database};

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
