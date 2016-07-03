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

fn main() {
  // runing server
  let mut ctrls = Vec::new();
  ctrls.push(controllers::get());

  server::start(&ctrls);
}