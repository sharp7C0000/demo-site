use iron::prelude::*;
use iron::status;
use server::controller::Controller;
use server::router::Router;

pub fn get() -> Controller{
	Controller::new(register)
}

fn register(router: &mut Router) {

	// GET : index
    router.add_route("".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world !")))
    });

	// GET : login
    router.add_route("login".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello login!")))
    });
}
