use nickel::{Nickel, StaticFilesHandler};

use iron::prelude::*;
use iron::status;

use std::path::Path;

use staticfile::Static;
use mount::Mount;

pub mod config;
pub mod controller;
pub mod router;

// constants
const CONFIG_FILE_PATH: &'static str = "Config.toml";

// start entry function
pub fn start(controllers: &Vec<controller::Controller>) {

	let mut server = Server::new();

    server.init();

	// register controllers
	for ctrl in controllers {
		let ctrl_regi_fn      = ctrl.get_register_fn();
		let mut nickel_server = &mut server.nickel_server;
		ctrl_regi_fn(nickel_server);
	}

	let listen_addr = "".to_string() + server.server_setting.get_ip() + ":" + server.server_setting.get_port();
    server.iron_server.http(&*listen_addr).unwrap();
}

pub struct Server {
	nickel_server : Nickel,
    iron_server   : Iron<Mount>,
	server_setting: config::ServerSetting
}

impl Server {
    pub fn new() -> Server {

        // test
        let mut router = router::Router::new();

        router.add_route("hello".to_string(), |_: &mut Request| {
            Ok(Response::with((status::Ok, "Hello world !")))
        });

        router.add_route("hello/again".to_string(), |_: &mut Request| {
           Ok(Response::with((status::Ok, "Hello again !")))
        });

        router.add_route("error".to_string(), |_: &mut Request| {
           Ok(Response::with(status::BadRequest))
        });

        //router.add_route("".to_string(), Static::new(Path::new("public")));

        let mut mount = Mount::new();
        // Serve the shared JS/CSS at /
        mount.mount("/assets", Static::new(Path::new("public/"))).mount("/", router);

        Server {
            nickel_server: Nickel::new(),
            iron_server: Iron::new(mount),
			server_setting: config::ServerSetting::new(CONFIG_FILE_PATH)
        }
    }

    pub fn init(&mut self) {

        let sv = &mut self.nickel_server;
        sv.utilize(StaticFilesHandler::new(self.server_setting.get_public_root()));
    }
}
