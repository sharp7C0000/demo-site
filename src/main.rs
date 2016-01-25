#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    server.get("/app", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("name", "user");
        return response.render("views/template.tpl", &data);
    });

    server.listen("127.0.0.1:6767");
}
