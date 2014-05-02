use collections::hashmap::HashMap;

pub struct Properties {
	props : HashMap<~str, ~str>
}

impl Properties {
	pub fn new() -> Properties {
		Properties { props : HashMap::new() }
	}
	pub fn setProperty(&mut self, k : ~str, v : ~str) {
		self.props.insert(k, v);
	}
	pub fn getProperty<'a>(&'a self, k : &~str) -> Option<&'a ~str> {
		self.props.find(k)
	}
	//fn store(&self) {}
}
