use iron::prelude::*;
use iron::status;
use iron_login::User;
use handlebars_iron::{Template, HandlebarsEngine, DirectorySource, MemorySource};

use server::controller::Controller;
use server::router::Router;

use std::collections::BTreeMap;


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

pub fn get() -> Controller{
	Controller::new(register)
}

fn register(router: &mut Router) {


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

	// GET : index
    router.add_route("".to_string(), |_: &mut Request| {
        let mut resp = Response::new();
        let mut data = BTreeMap::new();

        data.insert("year".to_string(), "2015".to_string());

        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
        Ok(resp)
    });

    router.add_route("login2".to_string(), request_handler);

	// GET : login
    router.add_route("login".to_string(), |_: &mut Request| {
        let mut resp = Response::new();
        let mut data = BTreeMap::new();

        data.insert("year".to_string(), "2015".to_string());

        resp.set_mut(Template::new("login", data)).set_mut(status::Ok);
        Ok(resp)
    });
}
