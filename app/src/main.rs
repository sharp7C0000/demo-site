#[macro_use] extern crate nickel;
#[macro_use] extern crate toml;

mod server;
mod controllers;

fn main() {
    // runing server
    let mut ctrls = Vec::new();
    ctrls.push(controllers::get());

    server::start(&ctrls);
}
