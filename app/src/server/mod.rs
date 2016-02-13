use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

pub mod router;

pub struct DemoServer {
	pub server: Nickel,
}

impl DemoServer {
    pub fn new() -> DemoServer {

        DemoServer {
            server: Nickel::new()
        }
    }

    // fn server(&mut self) -> &mut Nickel {
    //     &mut self.server
    // }
    //
    // fn utilize(&mut self) -> Nickel {
    //     self.server().utilize(StaticFilesHandler::new("public/"));
    //     self.server()
    // }

    pub fn init(&mut self) {
        let se2 = &mut self.server;

        se2
        .utilize(StaticFilesHandler::new("public/"));

		//self.register();



        //.listen("127.0.0.1:6767");

        // se.utilize(StaticFilesHandler::new("public/"));
        // se.listen("127.0.0.1:6767");
        // self.server;
    }

    pub fn start(&mut self) {


        let ref mut se2 = &mut self.server;
		//se2.listen("127.0.0.1:6767");




    }
}
