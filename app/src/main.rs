#[macro_use] extern crate nickel;
#[macro_use] extern crate toml;

mod server;

fn main() {
    // runing server
    server::start();
}
