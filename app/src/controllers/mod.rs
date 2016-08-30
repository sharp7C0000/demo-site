use iron::prelude::*;
use iron::status;
use iron_login::User;
use router::{Router};
use params::{Params, Value};
use handlebars_iron::{Template, HandlebarsEngine, DirectorySource, MemorySource};


use server::controller::Controller;
use models::SiteUser;

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

	// GET : index
  router.get("".to_string(), |_: &mut Request| {
    let mut resp = Response::new();
    resp.set_mut(Template::new("index", ())).set_mut(status::Ok);
    Ok(resp)
  });

	// GET : login
  router.get("login".to_string(), |_: &mut Request| {
    let mut resp = Response::new();
    resp.set_mut(Template::new("login", ())).set_mut(status::Ok);
    Ok(resp)
  });

  // POST : login
  router.post("login".to_string(), |req: &mut Request| {

    let map = req.get_ref::<Params>().unwrap();

    // check login form
    match (map.find(&["email"]), map.find(&["password"])) {
      (Some(&Value::String(ref name)), Some(&Value::String(ref password))) => {
          // database logic
          let mut resp = Response::new();
          resp.set_mut(status::Ok);
          Ok(resp)
      },
      _ => {
        let mut resp = Response::new();
        resp.set_mut(status::BadRequest);
        Ok(resp)
      }
    }
  });

  // GET : Admin page
  router.get("do-login".to_string(), |req: &mut Request| {
    let login = MyUser::get_login(req);
    let mut resp = Response::new();
    Ok(resp)
    // Ok(Response::new()
    //       .set(::iron::status::Ok)
    //       .set(format!("User set to '{}'", uid))
    //       .set(login.log_in(MyUser::new(uid))))
    // If a query (`?username`) is passed, set the username to that string
    // if let Some(ref uid) = req.url.query {
    //     // If no username is passed, log out
    //     if uid == "" {
    //       Ok(Response::new()
    //       .set(::iron::status::Ok)
    //       .set(format!("Logged out"))
    //       .set(login.log_out()))
    //     } else {
    //       Ok(Response::new()
    //       .set(::iron::status::Ok)
    //       .set(format!("User set to '{}'", uid))
    //       .set(login.log_in(MyUser::new(uid))))
    //     }
    // } else {
    //   let user = login.get_user();
    //   Ok(Response::new()
    //   .set(::iron::status::Ok)
    //   .set(format!("user = {:?}", user)))
    // }
  });

  // GET : Admin page
  router.get("admin".to_string(), |req: &mut Request| {
    let login = MyUser::get_login(req);

    match login.get_user() {
      // The division was valid
      Some(x) => {
        Ok(Response::new()
        .set(::iron::status::Ok)
        .set(format!("okok")))
      },
      // The division was invalid
      None    => {
        Ok(Response::new()
        .set(::iron::status::Ok)
        .set(format!("fuckoff")))
      },
    }
  });
}
