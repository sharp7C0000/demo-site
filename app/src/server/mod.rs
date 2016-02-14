use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

pub mod router;
pub mod config;

// start entry function
pub fn start() {

	let mut server = Server::new(

	);


    server.init();
    server.register();
    //server.start();
    server.server.listen("127.0.0.1:6767");
}

pub struct Server {
	server       : Nickel,
	serverSetting: config::ServerSetting
}

impl Server {
    pub fn new() -> Server {

        Server {
            server: Nickel::new(),
			serverSetting: config::ServerSetting::new("Config.toml")
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
