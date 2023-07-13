extern crate mongodb;

use std::result;
use mongodb::bson::Document;

use mongodb::{Client, options::ClientOptions, bson::doc};

#[tokio::main]
async fn main(){

    let client_options = ClientOptions::parse("mongodb://127.0.0.1:27017/testdb").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db_name = "testdb";
    let coll_name = "user";
    // create_collection(&client, db_name, coll_name).await;
    insert_document(&client, db_name, coll_name).await;
    // get_document(&client, db_name, coll_name).await;

}

async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    db.create_collection(coll_name, None).await.unwrap();
}

async fn insert_document(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    let coll = db.collection(coll_name);

    let doc = doc! { "name": "random", "age": 2125 };

    coll.insert_one(doc, None).await.unwrap();
}
// async fn get_document(client: &Client,db_name: &str,coll_name: &str){
//     let db = client.database(db_name);
//     let coll:= db.collection(coll_name);
//     let filter = doc! {"name":"john"};
//     let result = coll.find_one(filter, None).await.unwrap();

//     match result {

//         Some(doc) => println!("{}", doc),
//         None => println!("No document found"),

//     }



// }

// fn get_document(client: &Client, db_name: &str, coll_name: &str) {
//     let db = client.database(db_name);
//     let coll = db.collection(coll_name);

//     let filter = doc! {"name": "John"};

//     let result = coll.find_one(Some(filter), None);
//     match result {
//         Some(doc) => println!("{}", doc),
//         None => println!("No document found"),
//     }
// }

    