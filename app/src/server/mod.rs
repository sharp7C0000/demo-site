use iron::prelude::*;
use iron_login::User;
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

// test
#[derive(Debug)]
struct MyUser(String);
impl MyUser {
    fn new(user_id: &str) -> MyUser {
        MyUser(user_id.to_owned())
    }
}
impl User for MyUser {
    fn from_user_id(_: &mut Request, user_id: &str) -> Option<MyUser> {
        Some(MyUser(user_id.to_owned()))
    }
    fn get_user_id(&self) -> String {
        self.0.to_owned()
    }
}

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

        /// A basic iron request handler
fn request_handler(req: &mut Request) -> IronResult<Response> {
    let login = MyUser::get_login(req);
    // If a query (`?username`) is passed, set the username to that string
    if let Some(ref uid) = req.url.query {
        // If no username is passed, log out
        if uid == "" {
            Ok(Response::new()
                   .set(::iron::status::Ok)
                   .set(format!("Logged out"))
                   .set(login.log_out()))
        } else {
            Ok(Response::new()
                   .set(::iron::status::Ok)
                   .set(format!("User set to '{}'", uid))
                   .set(login.log_in(MyUser::new(uid))))
        }
    } else {
        let user = login.get_user();
        Ok(Response::new()
               .set(::iron::status::Ok)
               .set(format!("user = {:?}", user)))
    }
}

let cookie_signing_key = b"My Secret Key"[..].to_owned();

        // chaining middleware
        let mut chain = Chain::new(mount);

        //let mut chain2 = Chain::new(request_handler);
    chain.around(::iron_login::LoginManager::new(cookie_signing_key));

        chain.link_after(hbse);
        //chain.link_before(chain2);

        Server {
            iron_server: Iron::new(chain),
			server_setting: server_setting
        }
    }
}
