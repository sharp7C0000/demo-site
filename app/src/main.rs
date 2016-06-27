#[macro_use] extern crate iron;
#[macro_use] extern crate iron_login;
#[macro_use] extern crate staticfile;
#[macro_use] extern crate mount;
#[macro_use] extern crate handlebars_iron;
#[macro_use] extern crate toml;
#[macro_use] extern crate router;
#[macro_use] extern crate bson;
#[macro_use] extern crate mongodb;

mod server;
mod controllers;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::db::options::CreateCollectionOptions;

fn main() {
  // connect db
  let client = Client::connect("localhost", 27017)
    .ok().expect("Failed to initialize client.");

    let db = client.db("admin");

    db.auth("admin", "admin")
    .ok().expect("Failed to authorize user 'root'.");

    let db2 = client.db("dev");

let collections = db2.list_collections(None)
    .ok().expect("Are you sure you’ve been authorized?");

    db2.create_collection("comedies", None)
    .ok().expect("Failed to create 'comedies' collection!");

  // runing server
  let mut ctrls = Vec::new();
  ctrls.push(controllers::get());

  server::start(&ctrls);
}