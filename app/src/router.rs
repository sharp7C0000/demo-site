use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

pub fn register(server:&Nickel) {

    // server.get("/", middleware! { |_, response|
    //     let mut data = HashMap::new();
    //     data.insert("name", "user");
    //     return response.render("views/index.tpl", &data);
    // });
    //
    // println!("register");
}
