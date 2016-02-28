use nickel::Nickel;

pub struct Controller {
	register_fn : fn(server: &mut Nickel)
}

impl Controller {
	pub fn new(register_fn: fn(server: &mut Nickel)) -> Controller {
		Controller {
			register_fn: register_fn
		}
	}

	pub fn get_register_fn(&self) -> fn(server: &mut Nickel) { self.register_fn }
}
