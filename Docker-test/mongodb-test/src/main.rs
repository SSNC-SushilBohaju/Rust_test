extern crate mongodb;

use mongodb::{Client, options::ClientOptions};
// use tokio::prelude::*;

#[tokio::main]
async fn main(){
    let client_options = ClientOptions::parse("mongodb://127.0.0.1:27017/testdb").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db_name = "testdb";
    let coll_name = "mycollection";
    create_collection(&client, db_name, coll_name).await;
}
async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    db.create_collection(coll_name, None).await.unwrap();
}

