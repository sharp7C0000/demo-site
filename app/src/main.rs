#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

mod router;

struct DemoServer<'a> {
    server: &'a Nickel,
}

impl<'a> DemoServer<'a> {
    fn new(server: &'a Nickel) -> DemoServer<'a> {

        DemoServer {
            server: server
        }
    }

    fn server(&mut self) -> &'a Nickel {
        self.server
    }

    fn init(&mut self) {
        let mut se = self.server;
        //self.server.utilize(StaticFilesHandler::new("public/"));
        //se.listen("127.0.0.1:6767");
        //self.server;
    }
}


fn main() {
    let mut server = Nickel::new();

    //let mut se = DemoServer::new(&servers);

    //server.init();

    server.utilize(StaticFilesHandler::new("public/"));

    router::register(&server);

    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("name", "user");
        return response.render("views/index.tpl", &data);
    });

    server.listen("127.0.0.1:6767");


}
