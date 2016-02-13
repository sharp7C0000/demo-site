use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};
use server::DemoServer;

impl DemoServer {
    pub fn register(&mut self) {
		let se2 = &mut self.server;

        se2.get("/", middleware! { |_, response|
            let mut data = HashMap::new();
            data.insert("name", "user");
            return response.render("views/index.tpl", &data);
        });

        println!("register");
    }
}

// pub fn register(server:&Nickel) {
//
//     // server.get("/", middleware! { |_, response|
//     //     let mut data = HashMap::new();
//     //     data.insert("name", "user");
//     //     return response.render("views/index.tpl", &data);
//     // });
//     //
//     // println!("register");
// }
