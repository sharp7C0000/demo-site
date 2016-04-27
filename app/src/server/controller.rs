use iron::prelude::*;
use iron::status;
use server::router::Router;

pub struct Controller {
	register_fn : fn(router: &mut Router)
}

impl Controller {
	pub fn new(register_fn: fn(router: &mut Router)) -> Controller {
		Controller {
			register_fn: register_fn
		}
	}

	pub fn get_register_fn(&self) -> fn(router: &mut Router) { self.register_fn }
}
