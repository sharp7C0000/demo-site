use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::db::DatabaseInner;
use mongodb::db::options::CreateCollectionOptions;

use std::sync::Arc;

fn parse_port(port: &String) -> u16 {
  (*port).parse::<u16>().unwrap()
}

pub struct DB {
	connection: Arc<DatabaseInner>
}

impl DB {

  pub fn new(ip: &String, port: &String, db_name: &String) -> DB {
    let client = Client::connect(&*ip, parse_port(port))
    .ok()
    .expect("Failed to initialize client.");
    
    let connected_db:Arc<DatabaseInner> = client.db(db_name);

    DB {
      connection: connected_db
    }
  }

  pub fn new_with_auth(ip: &String, port: &String, db_name: &String, username: &String, password: &String) -> DB {
    let client = Client::connect(&*ip, parse_port(port))
    .ok()
    .expect("Failed to initialize client.");

    // auth
    let admin_db = client.db("admin");
    admin_db.auth(username, password)
    .ok()
    .expect("Failed to authorize");

    println!("test!!!!!! connected");

    let connected_db = client.db(db_name);
    let collections  = connected_db.list_collections(None)
    .ok().expect("Are you sure you’ve been authorized?");
    
    DB {
      connection: connected_db
    }
  }
}