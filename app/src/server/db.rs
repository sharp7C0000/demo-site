use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::db::options::CreateCollectionOptions;

pub fn connect(ip: &String, port: &String, dbName: &String) { 
  // connect db

  println!("hear!!!! {}", ip);
  


  let client = Client::connect(&*ip, (*port).parse::<u16>().unwrap())
  .ok().expect("Failed to initialize client.");

   let db2 = client.db(dbName);

   let collections = db2.list_collections(None)
   .ok().expect("Are you sure you’ve been authorized?");

  // db2.create_collection("comedies", None)
  // .ok().expect("Failed to create 'comedies' collection!");
}

pub fn connectWithAuth(ip: &String, port: u16, dbName: &String, username: &String, password: &String) { 
  // connect db
  // let client = Client::connect(&*ip, port)
  // .ok().expect("Failed to initialize client.");

  // let db = client.db("admin");

  // db.auth(username, password)
  // .ok().expect("Failed to authorize user 'root'.");

  // let db2 = client.db("dev");

  // let collections = db2.list_collections(None)
  // .ok().expect("Are you sure you’ve been authorized?");

  // db2.create_collection("comedies", None)
  // .ok().expect("Failed to create 'comedies' collection!");
}