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
const PUBLUC_ROUTE: &'static str = "assets";
const CONFIG_FILE_PATH: &'static str = "Config.toml";

// start entry function
pub fn start(controllers: &Vec<controller::Controller>) {

    let mut router = router::Router::new();

    // register routes
    for ctrl in controllers {
        let ctrl_regi_fn = ctrl.get_register_fn();
        ctrl_regi_fn(&mut router);
    }

	let mut server = Server::new(router);

	let listen_addr = "".to_string() + server.server_setting.get_ip() + ":" + server.server_setting.get_port();
    server.iron_server.http(&*listen_addr).unwrap();
}

pub struct Server {
	nickel_server : Nickel,
    iron_server   : Iron<Mount>,
	server_setting: config::ServerSetting
}

impl Server {
    pub fn new(rt: router::Router) -> Server {

        let server_setting = config::ServerSetting::new(CONFIG_FILE_PATH);
        let mut mount      = Mount::new();

        let pub_url  = format!("{}{}", server_setting.get_public_root(), "/");
        let pub_root = format!("{}{}", "/", PUBLUC_ROUTE.to_string());

        mount.mount(&pub_url, Static::new(Path::new(&pub_root))).mount("/", rt);

        Server {
            nickel_server: Nickel::new(),
            iron_server: Iron::new(mount),
			server_setting: server_setting
        }
    }
}
