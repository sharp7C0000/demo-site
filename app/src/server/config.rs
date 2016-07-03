use std::io::prelude::*;
use std::fs::File;

use std::error::Error;
use std::path::Path;

use toml;

pub struct DbAuth {
  // db user name
  username: String,
  // db password
  password: String
}

pub struct ServerSetting {
	// ip address
	ip  : String,
	// port number
	port: String,
	// public root
	public_root: String,
  // db ip address
  db_ip: String,
  // db port number
  db_port: String,
  // db name
  db_name: String,
  // db auth information
  db_auth: Option<DbAuth>
}

impl ServerSetting {

  pub fn new(config_file_path: &'static str) -> ServerSetting {

    // open config file
    let path    = Path::new(config_file_path);
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
    fn get_config_value(config: &toml::Value, lookup: &'static str) -> String {
      let result = match config.lookup(lookup) {
        Some(r) => r.as_str().unwrap(),
        None    => panic!("couldn't find config value {}", lookup),
      };

      String::from(result)
    };

    // get db auth setting
    let dbAuth = if let (Some(a), Some(b)) = (config.lookup("database.auth.username"), config.lookup("database.auth.password"))  {
      Some(DbAuth{
        username: String::from(a.as_str().unwrap()),
        password: String::from(b.as_str().unwrap())
      })
    } else {
      None
    };

    ServerSetting {
      ip         : get_config_value(config, "ip"),
      port       : get_config_value(config, "port"),
      public_root: get_config_value(config, "public_root"),
      db_ip      : get_config_value(config, "database.ip"),
      db_port    : get_config_value(config, "database.port"),
      db_name    : get_config_value(config, "database.db_name"),
      db_auth    : dbAuth
    }
  }

  pub fn get_ip(&self) -> &String { &self.ip }

  pub fn get_port(&self) -> &String { &self.port }

  pub fn get_public_root(&self) -> &String{ &self.public_root }

  pub fn get_db_ip(&self) -> &String { &self.db_ip }

  pub fn get_db_port(&self) -> &String { &self.db_port }
  
  pub fn get_db_name(&self) -> &String { &self.db_name }

  pub fn get_db_auth(&self) -> &Option<DbAuth> { &self.db_auth }
}
