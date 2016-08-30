use iron::prelude::*;
use iron::status;
use iron_login::User;

pub struct SiteUser {
	username: String,
  email   : String
}

impl SiteUser {
  fn new(username: &str, email: &str) -> SiteUser {
    SiteUser {
      username: username.to_owned(), 
      email: email.to_owned()
    }
  }
}

impl User for SiteUser {
  fn from_user_id(_: &mut Request, username: &str, email: &str) -> Option<SiteUser> {
    Some(SiteUser {
      username: username.to_owned(), 
      email: email.to_owned()}
    )
  }
  fn get_user_id(&self) -> String {
    self.0.to_owned()
  }
}