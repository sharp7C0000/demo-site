use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::db::options::CreateCollectionOptions;

// pub struct DB {
//   db: 
// }

impl Controller {
  pub fn new(register_fn: fn(router: &mut Router)) -> Controller {
    Controller {
      register_fn: register_fn
    }
  }

	pub fn get_register_fn(&self) -> fn(router: &mut Router) { self.register_fn }
}



fn parse_port(port: &String) -> u16 {
   (*port).parse::<u16>().unwrap()
}

pub fn connect(ip: &String, port: &String, dbName: &String) { 
  
  let client = Client::connect(&*ip, parse_port(port))
  .ok()
  .expect("Failed to initialize client.");

   let db2 = client.db(dbName);

   let collections = db2.list_collections(None)
   .ok().expect("Are you sure you’ve been authorized?");

  // db2.create_collection("comedies", None)
  // .ok().expect("Failed to create 'comedies' collection!");
}

pub fn connectWithAuth(ip: &String, port: &String, dbName: &String, username: &String, password: &String) { 
  
  let client = Client::connect(&*ip, parse_port(port))
  .ok()
  .expect("Failed to initialize client.");

  // auth
  let db = client.db("admin");
  db.auth(username, password)
  .ok()
  .expect("Failed to authorize");

  let db2 = client.db(dbName);

   

   let collections = db2.list_collections(None)
   .ok().expect("Are you sure you’ve been authorized?");

   println!("{}:?", db2);


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