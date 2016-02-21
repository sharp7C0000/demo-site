use std::io::prelude::*;
use std::fs::File;

use std::error::Error;
use std::path::Path;

use toml;

pub struct ServerSetting {
	// ip address
	ip  : String,
	// port number
	port: String,
	// public root
	public_root: String
}

impl ServerSetting {

	pub fn new(configFilePath: &'static str) -> ServerSetting {

		// open config file
		let path    = Path::new(configFilePath);
    	let display = path.display();

	    let mut file = match File::open(&path) {
	        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
	        Ok(file) => file,
	    };

		// read config file
	    let mut s = String::new();
	    match file.read_to_string(&mut s) {
	        Err(why)   => panic!("couldn't read {}: {}", display, Error::description(&why)),
	        Ok(result) => result,
	    };

		// parse toml
		let parsed = toml::Parser::new(&s).parse().unwrap();
		let config = parsed.get("config").unwrap();

		// assign config values
		fn getConfigValue(config: &toml::Value, lookup: &'static str) -> String {
			let result = match config.lookup(lookup) {
				Some(r) => r.as_str().unwrap(),
				None    => panic!("couldn't find config value {}", lookup),
			};

			String::from(result)
		};

		ServerSetting {
			ip         : getConfigValue(config, "ip"),
			port       : getConfigValue(config, "port"),
			public_root: getConfigValue(config, "public_root")
		}
	}

	pub fn get_ip(&self) -> &String { &self.ip }

	pub fn get_port(&self) -> &String { &self.port }

	pub fn get_public_root(&self) -> &String{ &self.public_root }

}
