use std::io::prelude::*;
use std::fs::File;
use toml;

use std::error::Error;
use std::path::Path;

pub struct ServerSetting {
	ip  : String,
	port: &'static str
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

		let value = toml::Parser::new(&s).parse().unwrap();
		println!("{:?}", value);

		let config = value.get("config").unwrap();

		let k = match config.lookup("ip").unwrap().as_str() {
			Some(s) => s,
			None => panic!("couldn't find ip address"),
		};
		//
		// println!("{:?}", k);

		let mut t = String::new();

		t.push_str(k);


		ServerSetting {
			ip: t,
			port: "1234"
		}


	}
}
