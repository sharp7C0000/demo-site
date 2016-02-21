use std::collections::HashMap;
use nickel::{Nickel, HttpRouter, StaticFilesHandler};

pub mod router;
pub mod config;

// constants
const CONFIG_FILE_PATH: &'static str = "Config.toml";

// start entry function
pub fn start() {

	let mut server = Server::new();

    server.init();
    server.register();

	let listen_addr = "".to_string() + server.serverSetting.get_ip() + ":" + server.serverSetting.get_port();
    server.nickelServer.listen(&*listen_addr);
}

pub struct Server {
	nickelServer : Nickel,
	serverSetting: config::ServerSetting
}

impl Server {
    pub fn new() -> Server {
        Server {
            nickelServer : Nickel::new(),
			serverSetting: config::ServerSetting::new(CONFIG_FILE_PATH)
        }
    }

    pub fn init(&mut self) {
        let sv = &mut self.nickelServer;
        sv.utilize(StaticFilesHandler::new(self.serverSetting.get_public_root()));
    }
}
