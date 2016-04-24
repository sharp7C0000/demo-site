#[macro_use] extern crate nickel; // remove?
#[macro_use] extern crate iron;
#[macro_use] extern crate staticfile;
#[macro_use] extern crate mount;
#[macro_use] extern crate toml;

mod server;
mod controllers;

fn main() {
    // runing server
    let mut ctrls = Vec::new();
    ctrls.push(controllers::get());

    server::start(&ctrls);
}
