use iron::prelude::*;
use iron::status;
use handlebars_iron::{Template, HandlebarsEngine, DirectorySource, MemorySource};

use server::controller::Controller;
use server::router::Router;

use std::collections::BTreeMap;

pub fn get() -> Controller{
	Controller::new(register)
}

fn register(router: &mut Router) {

	// GET : index
    router.add_route("".to_string(), |_: &mut Request| {
        let mut resp = Response::new();
        let mut data = BTreeMap::new();

        data.insert("year".to_string(), "2015".to_string());

        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
        Ok(resp)
    });

	// GET : login
    router.add_route("login".to_string(), |_: &mut Request| {
        let mut resp = Response::new();
        let mut data = BTreeMap::new();

        data.insert("year".to_string(), "2015".to_string());

        resp.set_mut(Template::new("login", data)).set_mut(status::Ok);
        Ok(resp)
    });
}
