use iron::prelude::*;
use iron::status;
use iron_login::User;

pub struct User {
	username: String
  email   : String
}

impl User {
  fn new(user_id: &str) -> MyUser {
      MyUser(user_id.to_owned())
  }
}

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