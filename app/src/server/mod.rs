use nickel::{Nickel, StaticFilesHandler};

pub mod config;
pub mod controller;

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
    server.nickel_server.listen(&*listen_addr);
}

pub struct Server {
	nickel_server : Nickel,
	server_setting: config::ServerSetting
}

impl Server {
    pub fn new() -> Server {
        Server {
            nickel_server : Nickel::new(),
			server_setting: config::ServerSetting::new(CONFIG_FILE_PATH)
        }
    }

    pub fn init(&mut self) {
        let sv = &mut self.nickel_server;
        sv.utilize(StaticFilesHandler::new(self.server_setting.get_public_root()));
    }
}
