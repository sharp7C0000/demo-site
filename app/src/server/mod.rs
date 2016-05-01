use iron::prelude::*;
use iron::status;

use std::path::Path;

use staticfile::Static;
use mount::Mount;

use handlebars_iron::{Template, HandlebarsEngine, DirectorySource, MemorySource};

pub mod config;
pub mod controller;
pub mod router;

// constants
const PUBLUC_ROUTE: &'static str = "assets";
const CONFIG_FILE_PATH: &'static str = "Config.toml";

// start entry function
pub fn start(controllers: &Vec<controller::Controller>) {

	let mut server = Server::new(controllers);

	let listen_addr = "".to_string() + server.server_setting.get_ip() + ":" + server.server_setting.get_port();
    server.iron_server.http(&*listen_addr).unwrap();
}

pub struct Server {
    iron_server   : Iron<Chain>,
	server_setting: config::ServerSetting
}

impl Server {
    pub fn new(ctrls: &Vec<controller::Controller>) -> Server {

        let mut router = router::Router::new();

        // register routes
        for ctrl in ctrls {
            let ctrl_regi_fn = ctrl.get_register_fn();
            ctrl_regi_fn(&mut router);
        }

        let server_setting = config::ServerSetting::new(CONFIG_FILE_PATH);
        let mut mount      = Mount::new();

        let pub_url  = format!("{}{}", server_setting.get_public_root(), "/");
        let pub_root = format!("{}{}", "/", PUBLUC_ROUTE.to_string());

        // register mount
        mount.mount(&pub_root, Static::new(Path::new(&pub_url))).mount("/", router);

        // register template engine
        let mut hbse = HandlebarsEngine::new();

        // add a directory source, all files with .hbs suffix will be loaded as template
        hbse.add(Box::new(DirectorySource::new("./views/", ".hbs")));

        // load templates from all registered sources
        if let Err(r) = hbse.reload() {
            panic!("{}", r);
        }

        // chaining middleware
        let mut chain = Chain::new(mount);
        chain.link_after(hbse);

        Server {
            iron_server: Iron::new(chain),
			server_setting: server_setting
        }
    }
}
