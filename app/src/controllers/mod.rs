use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};
use server::controller::Controller;

pub fn get() -> Controller{
	Controller::new(register)
}

fn register(nickel_server: &mut Nickel) {

	// GET : index
	nickel_server.get("/", middleware! { |_, response|
		let mut data = HashMap::new();
		data.insert("", "");
		return response.render("views/index.tpl", &data);
	});
}
